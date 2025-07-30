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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Style {
    Full,
    Link,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Markdown,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let url = args.url;
    let scraped = grab_url(url.as_str()).await?;
    format_response(scraped, args.style);

    Ok(())
}

fn format_response(infos: ScrapedWebpage, format: Style) {
    match format {
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
        format_response(webpage, Style::Full);
    }

    #[test]
    fn test_format_response_full_without_description() {
        let webpage = ScrapedWebpage {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            description: None,
            language: None,
        };

        format_response(webpage, Style::Full);
    }

    #[test]
    fn test_format_response_link_style() {
        let webpage = ScrapedWebpage {
            title: "Test Title".to_string(),
            url: "https://example.com".to_string(),
            description: Some("This description should be ignored".to_string()),
            language: Some("en".to_string()),
        };

        format_response(webpage, Style::Link);
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
        // When more formats are added, test them here
    }
}
