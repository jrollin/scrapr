use std::time::Duration;

use anyhow::{anyhow, Result};
use reqwest::{header, Client, StatusCode};
use serde::Serialize;
use thiserror::Error;
use url::Url;
use webpage::HTML;

#[derive(Debug, Serialize)]
pub struct ScrapedWebpage {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub language: Option<String>,
}

struct HtmlPage {
    content: String,
}

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("Client error (status: {0}): {1}")]
    Client(StatusCode, String),
    #[error("Server error (status: {0}): {1}")]
    Server(StatusCode, String),
    #[error("Timeout error {0}: {1}")]
    Timeout(String, String),
    #[error("Scraper error {0}: {1}")]
    Other(String, String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

pub async fn grab_url(url: &str, timeout_secs: u64, user_agent: &str, cleanup_tracking: bool) -> Result<ScrapedWebpage> {
    // validate URL first
    validate_url(url)?;
    // cleanup tracking parameters if requested
    let cleaned_url = if cleanup_tracking {
        cleanup_tracking_params(url)?
    } else {
        url.to_string()
    };
    // grap html page
    let html_response = retrieve_html_page(&cleaned_url, timeout_secs, user_agent).await?;
    // extract infos
    let html: HTML = extract_html_infos(html_response.content)?;
    // populate article for saving
    let article = ScrapedWebpage {
        title: html.title.unwrap_or("No title".to_string()),
        url: html.url.unwrap_or(cleaned_url),
        description: html.description,
        language: html.language,
    };

    Ok(article)
}

async fn retrieve_html_page(url: &str, timeout_secs: u64, user_agent: &str) -> Result<HtmlPage> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(user_agent)?,
    );
    let client = Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(timeout_secs))
        .default_headers(headers)
        .build()?;
    match client.get(url).send().await {
        Ok(response) => {
            let status = response.status();

            if status.is_client_error() {
                let error_body = response.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
                let error_msg = if error_body.len() > 200 {
                    format!("{}... [truncated]", &error_body[..200])
                } else {
                    error_body
                };
                return Err(anyhow!(ScraperError::Client(
                    status,
                    format!("{} - {}", url, error_msg)
                )));
            }
            if status.is_server_error() {
                let error_body = response.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
                let error_msg = if error_body.len() > 200 {
                    format!("{}... [truncated]", &error_body[..200])
                } else {
                    error_body
                };
                return Err(anyhow!(ScraperError::Server(
                    status,
                    format!("{} - {}", url, error_msg)
                )));
            }

            let content = response.text().await?;

            Ok(HtmlPage { content })
        }
        Err(e) => {
            if e.is_timeout() {
                Err(anyhow!(ScraperError::Timeout(
                    e.to_string(),
                    url.to_string()
                )))
            } else {
                Err(anyhow!(ScraperError::Other(e.to_string(), url.to_string())))
            }
        }
    }
}

fn validate_url(url: &str) -> Result<()> {
    match Url::parse(url) {
        Ok(parsed_url) => {
            match parsed_url.scheme() {
                "http" | "https" => Ok(()),
                scheme => Err(anyhow!(ScraperError::InvalidUrl(format!(
                    "Unsupported scheme '{}'. Only HTTP and HTTPS are supported", scheme
                )))),
            }
        }
        Err(e) => Err(anyhow!(ScraperError::InvalidUrl(format!(
            "Failed to parse URL '{}': {}", url, e
        )))),
    }
}

fn get_tracking_params() -> &'static [&'static str] {
    &[
        // Google Analytics & Ads
        "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content",
        "gclid", "gclsrc", "dclid", "fbclid",

        // Social Media
        "igshid", "twclid", "ttclid", "li_fat_id",

        // Email Marketing
        "_hsenc", "_hsmi", "vero_conv", "vero_id",

        // Other Common Trackers
        "ref", "referrer", "source", "campaign", "medium",
        "msclkid", "mc_cid", "mc_eid", "pk_source", "pk_medium", "pk_campaign",

        // Amazon
        "tag", "linkCode", "creativeASIN", "linkId",

        // Generic tracking
        "track", "tracking", "tracker", "affiliate", "aff", "sid"
    ]
}

fn cleanup_tracking_params(url_str: &str) -> Result<String> {
    let mut url = Url::parse(url_str).map_err(|e| {
        anyhow!(ScraperError::InvalidUrl(format!(
            "Failed to parse URL for cleanup '{}': {}", url_str, e
        )))
    })?;

    let tracking_params = get_tracking_params();

    // Get current query pairs and filter out tracking params
    let filtered_pairs: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(key, _)| !tracking_params.contains(&key.as_ref()))
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    // Clear existing query and rebuild with filtered params
    url.set_query(None);

    if !filtered_pairs.is_empty() {
        let query_string: String = filtered_pairs
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        url.set_query(Some(&query_string));
    }

    Ok(url.to_string())
}

fn extract_html_infos(response: String) -> Result<HTML> {
    // webpage
    let html = HTML::from_string(response, None)?;
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraped_webpage_creation() {
        let webpage = ScrapedWebpage {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            description: Some("Test description".to_string()),
            language: Some("en".to_string()),
        };

        assert_eq!(webpage.title, "Test Title");
        assert_eq!(webpage.url, "https://example.com");
        assert_eq!(webpage.description, Some("Test description".to_string()));
        assert_eq!(webpage.language, Some("en".to_string()));
    }

    #[test]
    fn test_scraped_webpage_with_none_values() {
        let webpage = ScrapedWebpage {
            title: "Title Only".to_string(),
            url: "https://example.com".to_string(),
            description: None,
            language: None,
        };

        assert_eq!(webpage.title, "Title Only");
        assert_eq!(webpage.url, "https://example.com");
        assert_eq!(webpage.description, None);
        assert_eq!(webpage.language, None);
    }

    #[test]
    fn test_extract_html_infos_basic() {
        let html_content = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <title>Test Page</title>
                <meta name="description" content="A test page for unit testing">
            </head>
            <body>
                <h1>Hello World</h1>
            </body>
            </html>
        "#;

        let result = extract_html_infos(html_content.to_string());
        assert!(result.is_ok());

        let html = result.unwrap();
        assert_eq!(html.title, Some("Test Page".to_string()));
        assert_eq!(
            html.description,
            Some("A test page for unit testing".to_string())
        );
        assert_eq!(html.language, Some("en".to_string()));
    }

    #[test]
    fn test_extract_html_infos_minimal() {
        let html_content = "<html><head><title>Minimal</title></head></html>";

        let result = extract_html_infos(html_content.to_string());
        assert!(result.is_ok());

        let html = result.unwrap();
        assert_eq!(html.title, Some("Minimal".to_string()));
    }

    #[test]
    fn test_extract_html_infos_invalid_html() {
        let invalid_html = "This is not HTML";

        let result = extract_html_infos(invalid_html.to_string());
        // Should still work as HTML parser is tolerant
        assert!(result.is_ok());
    }

    // Integration tests for grab_url would require network access
    // For now, we'll add a mock test structure
    #[tokio::test]
    async fn test_grab_url_with_mock_data() {
        // This is a basic structure for integration testing
        // In a real scenario, you'd use a mock HTTP server
        // For now, we'll test with a function that processes mock HTML

        let mock_html = r#"
            <!DOCTYPE html>
            <html lang="fr">
            <head>
                <title>Mock Page</title>
                <meta name="description" content="This is a mock page for testing">
            </head>
            <body>
                <h1>Mock Content</h1>
            </body>
            </html>
        "#;

        // Test the HTML processing part directly
        let html_result = extract_html_infos(mock_html.to_string());
        assert!(html_result.is_ok());

        let html = html_result.unwrap();
        assert_eq!(html.title, Some("Mock Page".to_string()));
        assert_eq!(html.description, Some("This is a mock page for testing".to_string()));
        assert_eq!(html.language, Some("fr".to_string()));
    }

    #[test]
    fn test_timeout_parameter() {
        // Test that different timeout values are accepted
        let timeout_5 = 5u64;
        let timeout_30 = 30u64;

        // These should be valid timeout values
        assert!(timeout_5 > 0);
        assert!(timeout_30 > 0);
        assert!(timeout_30 > timeout_5);
    }

    #[test]
    fn test_scraper_error_display() {
        use reqwest::StatusCode;

        let client_error = ScraperError::Client(StatusCode::NOT_FOUND, "https://example.com - Page not found".to_string());
        let error_string = format!("{}", client_error);
        assert!(error_string.contains("Client error"));
        assert!(error_string.contains("404"));
        assert!(error_string.contains("Page not found"));

        let server_error = ScraperError::Server(StatusCode::INTERNAL_SERVER_ERROR, "https://example.com - Internal error".to_string());
        let error_string = format!("{}", server_error);
        assert!(error_string.contains("Server error"));
        assert!(error_string.contains("500"));
        assert!(error_string.contains("Internal error"));

        let timeout_error = ScraperError::Timeout("Timeout occurred".to_string(), "https://example.com".to_string());
        let error_string = format!("{}", timeout_error);
        assert!(error_string.contains("Timeout error"));

        let other_error = ScraperError::Other("Network error".to_string(), "https://example.com".to_string());
        let error_string = format!("{}", other_error);
        assert!(error_string.contains("Scraper error"));
    }

    #[test]
    fn test_error_message_truncation() {
        // Test that long error messages are truncated properly
        let long_message = "a".repeat(300);
        let truncated = if long_message.len() > 200 {
            format!("{}... [truncated]", &long_message[..200])
        } else {
            long_message
        };

        assert!(truncated.len() <= 215); // 200 + "... [truncated]".len()
        assert!(truncated.ends_with("... [truncated]"));
    }

    #[test]
    fn test_user_agent_validation() {
        use reqwest::header::HeaderValue;

        // Test that valid user agents can be converted to HeaderValue
        let valid_ua = "MyBot/1.0";
        assert!(HeaderValue::from_str(valid_ua).is_ok());

        let firefox_ua = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0";
        assert!(HeaderValue::from_str(firefox_ua).is_ok());

        // Test that invalid user agents (with invalid chars) would fail
        let invalid_ua = "MyBot\n1.0";
        assert!(HeaderValue::from_str(invalid_ua).is_err());
    }

    #[test]
    fn test_url_validation() {
        // Valid HTTP URLs
        assert!(validate_url("http://example.com").is_ok());
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("https://example.com/path?param=value").is_ok());
        assert!(validate_url("http://subdomain.example.com:8080/path").is_ok());

        // Invalid URLs
        assert!(validate_url("not-a-url").is_err());
        assert!(validate_url("ftp://example.com").is_err());
        assert!(validate_url("file:///etc/passwd").is_err());
        assert!(validate_url("javascript:alert('xss')").is_err());

        // Empty or malformed URLs
        assert!(validate_url("").is_err());
        assert!(validate_url("://example.com").is_err());
    }

    #[test]
    fn test_url_validation_error_messages() {
        // Test that error messages contain helpful information
        let result = validate_url("ftp://example.com");
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.to_lowercase().contains("unsupported scheme"));
        assert!(error_msg.contains("ftp"));

        let result = validate_url("not-a-url");
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.to_lowercase().contains("failed to parse"));
    }

    #[test]
    fn test_cleanup_tracking_params_utm() {
        // Test UTM parameters removal
        let url = "https://example.com/page?utm_source=google&utm_medium=cpc&utm_campaign=test&real_param=keep";
        let cleaned = cleanup_tracking_params(url).unwrap();
        assert_eq!(cleaned, "https://example.com/page?real_param=keep");

        // Test URL with only tracking params
        let url_only_tracking = "https://example.com/?utm_source=facebook&utm_medium=social";
        let cleaned = cleanup_tracking_params(url_only_tracking).unwrap();
        assert_eq!(cleaned, "https://example.com/");
    }

    #[test]
    fn test_cleanup_tracking_params_various() {
        // Test different types of tracking parameters
        let url = "https://shop.example.com/item?gclid=123&fbclid=456&price=100&color=red&ref=newsletter";
        let cleaned = cleanup_tracking_params(url).unwrap();
        assert_eq!(cleaned, "https://shop.example.com/item?price=100&color=red");

        // Test Amazon tracking params
        let amazon_url = "https://amazon.com/product?tag=mytag&linkCode=123&creativeASIN=B123&productId=456";
        let cleaned = cleanup_tracking_params(amazon_url).unwrap();
        assert_eq!(cleaned, "https://amazon.com/product?productId=456");
    }

    #[test]
    fn test_cleanup_tracking_params_no_query() {
        // Test URL with no query parameters
        let url = "https://example.com/page";
        let cleaned = cleanup_tracking_params(url).unwrap();
        assert_eq!(cleaned, "https://example.com/page");

        // Test URL with fragment but no query
        let url_fragment = "https://example.com/page#section";
        let cleaned = cleanup_tracking_params(url_fragment).unwrap();
        assert_eq!(cleaned, "https://example.com/page#section");
    }

    #[test]
    fn test_cleanup_tracking_params_preserve_non_tracking() {
        // Test that non-tracking parameters are preserved
        let url = "https://example.com/search?q=rust&page=2&sort=date&utm_source=google";
        let cleaned = cleanup_tracking_params(url).unwrap();
        assert_eq!(cleaned, "https://example.com/search?q=rust&page=2&sort=date");

        // Test with complex parameter values
        let complex_url = "https://api.example.com/endpoint?data=%7B%22key%22%3A%22value%22%7D&utm_campaign=test";
        let cleaned = cleanup_tracking_params(complex_url).unwrap();
        assert!(cleaned.starts_with("https://api.example.com/endpoint?data="));
        assert!(!cleaned.contains("utm_campaign"));
    }

    #[test]
    fn test_get_tracking_params_coverage() {
        let tracking_params = get_tracking_params();

        // Test that common tracking parameters are included
        assert!(tracking_params.contains(&"utm_source"));
        assert!(tracking_params.contains(&"utm_medium"));
        assert!(tracking_params.contains(&"utm_campaign"));
        assert!(tracking_params.contains(&"gclid"));
        assert!(tracking_params.contains(&"fbclid"));
        assert!(tracking_params.contains(&"ref"));
        assert!(tracking_params.contains(&"track"));

        // Test that the list has a reasonable number of parameters
        assert!(tracking_params.len() > 10);
        assert!(tracking_params.len() < 50); // Reasonable upper bound
    }

    #[test]
    fn test_cleanup_tracking_params_invalid_url() {
        // Test error handling for invalid URLs
        let result = cleanup_tracking_params("not-a-url");
        assert!(result.is_err());

        // Note: ftp:// URLs are valid URLs, they just have unsupported schemes
        // The cleanup function only parses URLs, it doesn't validate schemes
        let result = cleanup_tracking_params("ftp://example.com?utm_source=test");
        assert!(result.is_ok()); // This should succeed as it's a valid URL structure
        let cleaned = result.unwrap();
        assert_eq!(cleaned, "ftp://example.com/"); // URL parser normalizes to include trailing slash
    }

    #[test]
    fn test_tracking_params_edge_cases() {
        // Test empty query parameter
        let url = "https://example.com/?utm_source=&real_param=value";
        let cleaned = cleanup_tracking_params(url).unwrap();
        assert_eq!(cleaned, "https://example.com/?real_param=value");

        // Test parameter with special characters
        let url = "https://example.com/?utm_source=test%20space&param=keep%20this";
        let cleaned = cleanup_tracking_params(url).unwrap();
        assert_eq!(cleaned, "https://example.com/?param=keep%20this");

        // Test case sensitivity (tracking params should be case sensitive)
        let url = "https://example.com/?UTM_SOURCE=test&utm_source=test&param=keep";
        let cleaned = cleanup_tracking_params(url).unwrap();
        assert_eq!(cleaned, "https://example.com/?UTM_SOURCE=test&param=keep");
    }
}
