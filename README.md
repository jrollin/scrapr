# Scrapr CLI

[![CI](https://github.com/jrollin/scrapr/workflows/CI/badge.svg)](https://github.com/jrollin/scrapr/actions/workflows/ci.yml)
[![Release](https://github.com/jrollin/scrapr/workflows/Release/badge.svg)](https://github.com/jrollin/scrapr/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)

🦀 A fast, reliable Rust command-line tool for extracting structured content from web pages with intelligent tracking parameter cleanup.

## Overview

Scrapr streamlines content creation workflows by automatically extracting key metadata from web pages (title, description, URL) and formatting it as clean markdown. Perfect for bloggers, journalists, researchers, and content creators who need to quickly generate reference links without tracking cruft.

## ✨ Features

### Core Functionality
- **⚡ Fast Web Scraping**: Asynchronous HTTP requests with gzip compression
- **🧠 Intelligent Content Extraction**: Automatically extracts page title, description, and language metadata
- **🎨 Multiple Output Formats**: Markdown (default) and JSON output support
- **📝 Flexible Output Styles**: Choose between full content with descriptions or simple link format

### Advanced Features
- **🧹 Smart URL Cleanup**: Automatically removes tracking parameters (UTM, Google Analytics, Facebook, Amazon affiliate, etc.)
- **⚙️ Configurable Settings**: Custom timeouts, user agents, and cleanup behavior
- **🛡️ Robust Error Handling**: Comprehensive error handling with detailed context for timeouts, HTTP errors, and invalid URLs
- **🔍 URL Validation**: Validates URLs before processing and supports only HTTP/HTTPS schemes

## 📦 Installation

### Prerequisites

- **Rust 1.70+** (for building from source)
- **Linux/macOS/Windows** (binary availability varies)

### Option 1: Install from Crates.io (Recommended)

**Note:** Package not yet published to crates.io. Please use Option 3 (Build from Source) for now.

```bash
# Install directly from crates.io (when available)
cargo install scrapr

# Verify installation
scrapr --version
```

### Option 2: Pre-built Binaries

**Note:** Pre-built binaries are not yet available. Please use the source installation method below.

<!-- Future binary installation when releases are available:
```bash
# Linux x86_64
curl -L https://github.com/jrollin/scrapr/releases/latest/download/scrapr-x86_64-unknown-linux-gnu -o scrapr
chmod +x scrapr
sudo mv scrapr /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/jrollin/scrapr/releases/latest/download/scrapr-x86_64-apple-darwin -o scrapr
chmod +x scrapr
sudo mv scrapr /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/jrollin/scrapr/releases/latest/download/scrapr-aarch64-apple-darwin -o scrapr
chmod +x scrapr
sudo mv scrapr /usr/local/bin/
```
-->

### Option 3: Build from Source

```bash
# Clone the repository
git clone https://github.com/jrollin/scrapr.git
cd scrapr

# Build the release binary
cargo build --release

# Install to system PATH
sudo cp target/release/scrapr /usr/local/bin/

# Or install to user directory (no sudo required)
mkdir -p ~/.local/bin
cp target/release/scrapr ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
scrapr --version
```

### Option 4: Run with Cargo (Development)

```bash
# Clone and run directly
git clone https://github.com/jrollin/scrapr.git
cd scrapr

# Run without installing
cargo run -- --url https://example.com
```

### Package Manager Installation (Future)

```bash
# Homebrew (macOS/Linux) - Coming soon
brew install scrapr

# Arch Linux AUR - Coming soon
yay -S scrapr

# Debian/Ubuntu - Coming soon
sudo apt install scrapr
```

## 🚀 Usage

### Command Line Options

```bash
Usage: scrapr [OPTIONS] --url <URL>

Options:
  -u, --url <URL>                Target URL to scrape
  -s, --style <STYLE>            Output style [default: full] [possible values: full, link]
  -f, --format <FORMAT>          Output format [default: markdown] [possible values: markdown, json]
  -t, --timeout <TIMEOUT>        HTTP request timeout in seconds [default: 10]
      --user-agent <USER_AGENT>  Custom User-Agent header [default: Firefox 116]
      --cleanup-tracking         Remove tracking query parameters [default: enabled]
      --no-cleanup-tracking      Disable tracking parameter cleanup
  -h, --help                     Print help information
  -V, --version                  Print version information
```

### 📖 Examples

#### Basic Usage

**Extract with full description (default):**
```bash
scrapr --url https://www.rust-lang.org/

# Output:
- [Rust Programming Language](https://www.rust-lang.org/)\
A language empowering everyone to build reliable and efficient software.
```

**Link only format:**
```bash
scrapr --url https://www.rust-lang.org/ --style link

# Output:
[Rust Programming Language](https://www.rust-lang.org/)
```

#### Advanced Usage

**JSON output format:**
```bash
scrapr --url https://example.com --format json

# Output:
{
  "title": "Example Domain",
  "url": "https://example.com",
  "description": "Example website description",
  "language": "en"
}
```

**Custom timeout and user agent:**
```bash
scrapr --url https://slow-site.com --timeout 30 --user-agent "MyBot/1.0"
```

#### URL Cleanup Examples

**Automatic tracking parameter removal (default):**
```bash
# Input URL with tracking
scrapr --url "https://example.com/article?utm_source=google&utm_medium=cpc&product_id=123"

# Clean URL in output (tracking params removed):
- [Article Title](https://example.com/article?product_id=123)\
Article description here.
```

**Preserve all parameters:**
```bash
scrapr --url "https://example.com/?utm_source=newsletter&page=2" --no-cleanup-tracking

# Output preserves all parameters:
- [Page Title](https://example.com/?utm_source=newsletter&page=2)\
Page description here.
```

### 🧹 Tracking Parameters Cleaned

Scrapr automatically removes these common tracking parameters:

| Category | Parameters |
|----------|------------|
| **Google Analytics** | `utm_source`, `utm_medium`, `utm_campaign`, `utm_term`, `utm_content` |
| **Google/Facebook Ads** | `gclid`, `gclsrc`, `dclid`, `fbclid` |
| **Social Media** | `igshid`, `twclid`, `ttclid`, `li_fat_id` |
| **Email Marketing** | `_hsenc`, `_hsmi`, `vero_conv`, `vero_id` |
| **Amazon Affiliate** | `tag`, `linkCode`, `creativeASIN`, `linkId` |
| **Generic Tracking** | `ref`, `referrer`, `source`, `campaign`, `medium`, `track`, `tracking`, `tracker`, `affiliate`, `aff`, `sid` |

## 🛠️ Technical Details

### Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| **reqwest** | HTTP client with async support and gzip compression | 0.12+ |
| **webpage** | HTML parsing and metadata extraction | 2.0+ |
| **clap** | Command-line argument parsing with derive macros | 4.4+ |
| **tokio** | Async runtime for concurrent operations | 1.32+ |
| **serde** | Serialization framework for JSON output | 1.0+ |
| **url** | URL parsing and validation | 2.5+ |
| **anyhow/thiserror** | Ergonomic error handling | 1.0+ |

### Architecture

```
src/
├── main.rs    # CLI interface, argument parsing, and application entry point
└── scrap.rs   # Core scraping logic, HTTP client, and URL processing
```

### Error Handling

Scrapr provides comprehensive error handling for:

| Error Type | Description | HTTP Status |
|------------|-------------|-------------|
| **Client Errors** | Bad requests, authentication issues, not found | 4xx codes |
| **Server Errors** | Internal server errors, service unavailable | 5xx codes |
| **Network Timeouts** | Configurable timeout (default: 10s) | - |
| **Invalid URLs** | Malformed URLs, unsupported schemes | - |
| **Parsing Errors** | HTML parsing failures, metadata extraction issues | - |

Error messages include response body context (truncated to 200 characters) for better debugging.

## 🎯 Use Cases

### Content Creation
- **📝 Blog Writing**: Generate clean markdown references for static site generators (Hugo, Jekyll, Gatsby)
- **📚 Research**: Create formatted citations from web sources without tracking noise
- **📖 Documentation**: Build link lists with descriptions for project documentation

### Data Processing
- **🧹 URL Cleaning**: Remove tracking parameters from URLs in bulk processing
- **📊 Content Analysis**: Extract structured metadata for content auditing
- **🔗 Link Validation**: Verify and clean URLs before storage or sharing

### Integration
- **⚙️ IDE Integration**: Use as a command-line tool in editors and IDEs
- **🤖 Automation**: Integrate into content pipelines and automation scripts
- **📋 Note Taking**: Quick reference generation for knowledge management systems

## 🗺️ Roadmap

### Near Term
- [x] ~~JSON output format support~~
- [x] ~~Configurable timeout settings~~
- [x] ~~Custom User-Agent support~~
- [x] ~~Tracking parameter cleanup~~
- [ ] YAML output format
- [ ] Configuration file support (.scrapr.toml)

### Future Features
- [ ] Batch processing multiple URLs from file/stdin
- [ ] Custom tracking parameter lists
- [ ] Template-based output formatting
- [ ] Image and media metadata extraction
- [ ] RSS/Atom feed parsing support
- [ ] Plugin system for custom extractors

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- HTML parsing powered by [html5ever](https://github.com/servo/html5ever)
- HTTP client built on [reqwest](https://github.com/seanmonstar/reqwest)
- CLI parsing with [clap](https://github.com/clap-rs/clap)
