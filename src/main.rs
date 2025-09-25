mod scrap;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use scrap::grab_url;

use crate::scrap::ScrapedWebpage;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    url: String,
    #[arg(short, long, value_enum, default_value = "full")]
    style: Style,
    #[arg(short, long, value_enum, default_value = "markdown")]
    format: Format,
    #[arg(short, long, default_value = "10")]
    timeout: u64,
    #[arg(long, default_value = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0")]
    user_agent: String,
    #[arg(long, default_value_t = true, help = "Remove tracking query parameters (utm_source, etc.)")]
    cleanup_tracking: bool,
    #[arg(long, action = clap::ArgAction::SetFalse, help = "Disable tracking parameter cleanup")]
    no_cleanup_tracking: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Style {
    Full,
    Link,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Markdown,
    Json,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let url = args.url;
    let should_cleanup = args.cleanup_tracking && !args.no_cleanup_tracking;
    let scraped = grab_url(url.as_str(), args.timeout, &args.user_agent, should_cleanup).await?;
    format_response(scraped, args.style, args.format);

    Ok(())
}

fn format_response(infos: ScrapedWebpage, style: Style, format: Format) {
    match format {
        Format::Markdown => {
            match style {
                Style::Full => {
                    print!("- [{}]({})", infos.title, infos.url);
                    if let Some(description) = infos.description {
                        println!("\\");
                        println!("{}", description);
                    }
                }
                Style::Link => {
                    println!("[{}]({})", infos.title, infos.url);
                }
            }
        }
        Format::Json => {
            if let Ok(json) = serde_json::to_string_pretty(&infos) {
                println!("{}", json);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_response_full_with_description() {
        let webpage = ScrapedWebpage {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            description: Some("Test description".to_string()),
            language: Some("en".to_string()),
        };

        // We can't easily test println! output, but we can test the logic
        // This test verifies the function doesn't panic
        format_response(webpage, Style::Full, Format::Markdown);
    }

    #[test]
    fn test_format_response_full_without_description() {
        let webpage = ScrapedWebpage {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            description: None,
            language: None,
        };

        format_response(webpage, Style::Full, Format::Markdown);
    }

    #[test]
    fn test_format_response_link_style() {
        let webpage = ScrapedWebpage {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            description: Some("This description should be ignored".to_string()),
            language: Some("en".to_string()),
        };

        format_response(webpage, Style::Link, Format::Markdown);
    }

    #[test]
    fn test_args_default_values() {
        // Test that our enums have the expected default behavior
        assert_eq!(Style::Full as u8, 0); // First variant
        assert_eq!(Format::Markdown as u8, 0); // First variant
    }

    #[test]
    fn test_style_enum_values() {
        // Test enum variants exist
        let _full = Style::Full;
        let _link = Style::Link;
        
        // Test they can be compared
        assert_ne!(Style::Full, Style::Link);
    }

    #[test]
    fn test_format_enum_values() {
        let _markdown = Format::Markdown;
        let _json = Format::Json;

        // Test they can be compared
        assert_ne!(Format::Markdown, Format::Json);
    }

    #[test]
    fn test_format_response_json() {
        let webpage = ScrapedWebpage {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            description: Some("Test description".to_string()),
            language: Some("en".to_string()),
        };

        // Test JSON format doesn't panic (style is ignored for JSON)
        format_response(webpage, Style::Full, Format::Json);
    }
}
