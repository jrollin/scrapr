use std::time::Duration;

use anyhow::{anyhow, Result};
use reqwest::{header, Client, StatusCode};
use serde::Serialize;
use thiserror::Error;
use webpage::HTML;

#[derive(Debug, Serialize)]
pub struct ScrapedWebpage {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub language: Option<String>,
}

struct HtmlPage {
    url: String,
    content: String,
}

#[derive(Debug, Error)]
pub enum ScraperError {
    #[error("Client error (status: {0}): {1}")]
    Client(StatusCode, String),
    #[error("Server error (status: {0}): {1}")]
    Server(StatusCode, String),
    #[error("Timout error {0}: {1}")]
    Timeout(String, String),
    #[error("Scraper error {0}: {1}")]
    Other(String, String),
}

pub async fn grab_url(url: &str) -> Result<ScrapedWebpage> {
    // grap html page
    let html_response = retrieve_html_page(url).await?;
    // extract infos
    let html: HTML = extract_html_infos(html_response.content)?;
    // populate article for saving
    let article = ScrapedWebpage {
        title: html.title.unwrap_or("No title".to_string()),
        url: html.url.unwrap_or(html_response.url),
        description: html.description,
        language: html.language,
    };

    Ok(article)
}

async fn retrieve_html_page(url: &str) -> Result<HtmlPage> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0",
        ),
    );
    let client = Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(2))
        .default_headers(headers)
        .build()?;
    match client.get(url).send().await {
        Ok(response) => {
            if response.status().is_client_error() {
                return Err(anyhow!(ScraperError::Client(
                    response.status(),
                    url.to_string()
                )));
            }
            if response.status().is_server_error() {
                return Err(anyhow!(ScraperError::Server(
                    response.status(),
                    url.to_string()
                )));
            }

            let url = response.url().to_string();
            let content = response.text().await?;

            Ok(HtmlPage { url, content })
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
    fn test_scraper_error_display() {
        use reqwest::StatusCode;
        
        let client_error = ScraperError::Client(StatusCode::NOT_FOUND, "https://example.com".to_string());
        let error_string = format!("{}", client_error);
        assert!(error_string.contains("Client error"));
        assert!(error_string.contains("404"));
        
        let server_error = ScraperError::Server(StatusCode::INTERNAL_SERVER_ERROR, "https://example.com".to_string());
        let error_string = format!("{}", server_error);
        assert!(error_string.contains("Server error"));
        assert!(error_string.contains("500"));
        
        let timeout_error = ScraperError::Timeout("Timeout occurred".to_string(), "https://example.com".to_string());
        let error_string = format!("{}", timeout_error);
        assert!(error_string.contains("Timout error"));
        
        let other_error = ScraperError::Other("Network error".to_string(), "https://example.com".to_string());
        let error_string = format!("{}", other_error);
        assert!(error_string.contains("Scraper error"));
    }
}
