# Scrapr CLI

🦀 A fast and reliable Rust command-line tool for extracting structured content from web pages and generating markdown-formatted output.

## Overview

Scrapr is designed to streamline content creation workflows by automatically extracting key metadata from web pages (title, description, URL) and formatting it as markdown. Perfect for bloggers, journalists, and content creators who need to quickly generate reference links and summaries.

## Features

- **Fast Web Scraping**: Asynchronous HTTP requests with gzip compression support
- **Intelligent Content Extraction**: Automatically extracts page title, description, and language metadata
- **Multiple Output Styles**: Choose between full content with descriptions or simple link format
- **Error Handling**: Robust error handling for timeouts, client errors, and server errors
- **User-Agent Spoofing**: Uses realistic browser user-agent for better compatibility

## Installation

### Binary Installation (Linux)

**Download and install the binary:**
```bash
# Download the latest release binary
wget https://github.com/<username>/rust-scrapr/releases/latest/download/scrapr-x86_64-unknown-linux-gnu

# Make it executable
chmod +x scrapr-x86_64-unknown-linux-gnu

# Move to a directory in your PATH
sudo mv scrapr-x86_64-unknown-linux-gnu /usr/local/bin/scrapr

# Verify installation
scrapr --version
```

**Alternative installation locations:**
```bash
# Install to ~/.local/bin (user-specific, no sudo required)
mkdir -p ~/.local/bin
mv scrapr-x86_64-unknown-linux-gnu ~/.local/bin/scrapr
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or install to ~/bin
mkdir -p ~/bin
mv scrapr-x86_64-unknown-linux-gnu ~/bin/scrapr
echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### From Source

```bash
git clone <repository-url>
cd rust-scrapr
cargo build --release

# The binary will be at target/release/scrapr
# You can copy it to your PATH:
sudo cp target/release/scrapr /usr/local/bin/
```

### Development Mode

```bash
cargo run -- --url <URL> [OPTIONS]
```

## Usage

```bash
Usage: scrapr [OPTIONS] --url <URL>

Options:
  -u, --url <URL>        Target URL to scrape
  -s, --style <STYLE>    Output style [default: full] [possible values: full, link]
  -f, --format <FORMAT>  Output format [default: markdown] [possible values: markdown]
  -h, --help             Print help information
  -V, --version          Print version information
```

### Examples

**Full content with description:**

```bash
# Using installed binary
scrapr --url https://www.rust-lang.org/

# Or with cargo run
cargo run -- --url https://www.rust-lang.org/

# Output:
- [Rust Programming Language](https://www.rust-lang.org/)\
A language empowering everyone to build reliable and efficient software.
```

**Link only:**

```bash
# Using installed binary
scrapr --url https://www.rust-lang.org/ --style link

# Or with cargo run
cargo run -- --url https://www.rust-lang.org/ --style link

# Output:
[Rust Programming Language](https://www.rust-lang.org/)
```

## Technical Details

### Dependencies

- **reqwest**: HTTP client with async support and gzip compression
- **webpage**: HTML parsing and metadata extraction
- **clap**: Command-line argument parsing
- **tokio**: Async runtime
- **serde**: Serialization framework
- **anyhow/thiserror**: Error handling

### Architecture

- `main.rs`: CLI interface and argument parsing
- `scrap.rs`: Core scraping logic and HTTP client implementation

### Error Handling

The tool handles various error scenarios:

- Client errors (4xx status codes)
- Server errors (5xx status codes)
- Network timeouts (2-second timeout)
- Invalid URLs and parsing errors

## Use Cases

- **Blog Writing**: Quickly generate markdown references for Hugo/Jekyll sites
- **Research**: Create formatted citations from web sources
- **Documentation**: Generate link lists with descriptions
- **Content Curation**: Build reading lists and resource collections

## Roadmap

- [ ] Support for additional output formats (JSON, YAML, HTML)
- [ ] Custom styling templates
- [ ] Batch processing multiple URLs
- [ ] Configuration file support
- [ ] Image and media extraction
- [ ] RSS/Atom feed parsing
