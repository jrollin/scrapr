# Scrapr cli

🦀 A rust command line tool to create enriched content from URL

## Motivation

Personal project to write my news articles easier
I use Hugo website with markdown files

## Usage

```bash
Usage: scrapr [OPTIONS] --url <URL>

Options:
  -u, --url <URL>
  -s, --style <STYLE>    [default: full] [possible values: full, link]
  -f, --format <FORMAT>  [default: markdown] [possible values: markdown]
  -h, --help             Print help
  -V, --version          Print version
```

Create a Link with content

```bash
cargo run -- --url http://www.rustlang.com

- [Rust Programming Language](https://www.rust-lang.org/)\
A language empowering everyone to build reliable and efficient software.
```

Create only a Link

```bash
cargo run -- --url http://www.rustlang.com --style link

[Rust Programming Language](https://www.rust-lang.org/)
```

## TODO

- add more formats and style
