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
