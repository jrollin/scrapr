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
