# mcp-browser

[![Crates.io](https://img.shields.io/crates/v/mcp-browser.svg)](https://crates.io/crates/mcp-browser)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://www.zavora.ai)

Web browsing and scraping for AI agents — read pages as clean text, take screenshots, extract structured data, interact with forms, and get accessibility trees. **17 tools** powered by headless Chrome via native CDP (Chrome DevTools Protocol).

![Architecture](https://raw.githubusercontent.com/zavora-ai/mcp-browser/main/docs/assets/architecture.svg)

## Why mcp-browser?

- **Zero config** — Chrome auto-downloads and launches on first run
- **Full rendering** — JavaScript-heavy SPAs work out of the box
- **Dual mode** — headless Chrome for complex pages, lightweight HTTP for simple ones
- **Structured extraction** — CSS selectors, accessibility trees, metadata
- **Visual capture** — screenshots and PDFs
- **Form interaction** — click, type, wait, evaluate JS

## Installation

```bash
cargo install mcp-browser
```

Or build from source:

```bash
git clone https://github.com/zavora-ai/mcp-browser.git
cd mcp-browser
cargo build --release
```

## Configuration

### Claude Desktop / Cursor / any MCP client

```json
{
  "mcpServers": {
    "browser": {
      "command": "mcp-browser"
    }
  }
}
```

No environment variables or API keys required. Chrome is managed automatically.



## Tools (17)

### Read & Extract

| Tool | Description |
|------|-------------|
| `read_page` | Fetch URL → clean readable text (strips HTML, scripts, styles) |
| `get_html` | Full rendered HTML after JavaScript execution |
| `extract_links` | All links with text and href (up to 100) |
| `extract_metadata` | Title, description, OG tags, canonical URL |
| `query_selector` | Extract text content of elements matching CSS selector |
| `get_accessibility_tree` | Structured page representation (77% fewer tokens than HTML) |
| `extract_structured` | Extract named fields using a selector map |

### Visual

| Tool | Description |
|------|-------------|
| `screenshot` | Full-page screenshot (base64 PNG) |
| `screenshot_element` | Screenshot a specific element by CSS selector |
| `save_pdf` | Save page as PDF (base64) |

### Interact

| Tool | Description |
|------|-------------|
| `click` | Click an element by CSS selector |
| `type_text` | Type text into an input field |
| `evaluate_js` | Execute JavaScript and return the result |
| `wait_for` | Wait for an element to appear (with timeout) |

### Navigate

| Tool | Description |
|------|-------------|
| `navigate` | Go to URL, return title + load time + size |
| `check_status` | HTTP HEAD check (fast, no Chrome needed) |
| `fetch_light` | Lightweight fetch without Chrome (for simple/static pages) |

## Usage Examples with Sample Output

### Reading a page as clean text

```
Tool: read_page
Input: {"url": "https://example.com"}
```

Output:
```
Example Domain This domain is for use in illustrative examples in documents.
You may use this domain in literature without prior coordination or asking
for permission. More information...
```

### Extracting structured data from a product page

```
Tool: extract_structured
Input: {
  "url": "https://books.toscrape.com/catalogue/a-light-in-the-attic_1000/index.html",
  "fields": {
    "title": "h1",
    "price": ".price_color",
    "availability": ".availability",
    "description": "#product_description ~ p"
  }
}
```

Output:
```json
{
  "title": "A Light in the Attic",
  "price": "£51.77",
  "availability": "In stock (22 available)",
  "description": "It's hard to imagine a world without A Light in the Attic..."
}
```

### Getting page metadata

```
Tool: extract_metadata
Input: {"url": "https://github.com"}
```

Output:
```json
{
  "title": "GitHub: Let's build from here · GitHub",
  "description": "GitHub is where over 100 million developers shape the future of software...",
  "og_title": "GitHub: Let's build from here",
  "og_image": "https://github.githubassets.com/assets/campaign-social-031d6161fa10.png",
  "canonical": "https://github.com/"
}
```

### Extracting all links

```
Tool: extract_links
Input: {"url": "https://news.ycombinator.com"}
```

Output:
```json
[
  {"href": "https://news.ycombinator.com/newest", "text": "new"},
  {"href": "https://news.ycombinator.com/front", "text": "past"},
  {"href": "https://news.ycombinator.com/newcomments", "text": "comments"},
  {"href": "https://example.com/article", "text": "Show HN: My new project"},
  ...
]
```

### Taking a screenshot

```
Tool: screenshot
Input: {"url": "https://example.com"}
```

Output:
```json
{
  "format": "png",
  "size_bytes": 45231,
  "base64": "iVBORw0KGgoAAAANSUhEUgAA..."
}
```

### Checking page status (fast, no Chrome)

```
Tool: check_status
Input: {"url": "https://api.github.com"}
```

Output:
```json
{
  "url": "https://api.github.com",
  "status": 200,
  "content_type": "application/json; charset=utf-8",
  "content_length": null
}
```

### Navigating and measuring load time

```
Tool: navigate
Input: {"url": "https://www.rust-lang.org"}
```

Output:
```json
{
  "url": "https://www.rust-lang.org",
  "title": "Rust Programming Language",
  "load_time_ms": 1243,
  "size_bytes": 19847
}
```

### CSS selector queries

```
Tool: query_selector
Input: {"url": "https://news.ycombinator.com", "selector": ".titleline > a"}
```

Output:
```json
[
  "Show HN: I built a tool that...",
  "The future of programming languages",
  "Why Rust is taking over systems programming",
  ...
]
```

### Getting the accessibility tree

```
Tool: get_accessibility_tree
Input: {"url": "https://example.com"}
```

Output:
```json
{
  "role": "body",
  "name": "",
  "children": [
    {
      "role": "div",
      "name": "",
      "children": [
        {"role": "h1", "name": "Example Domain"},
        {"role": "p", "name": "This domain is for use in illustrative e..."},
        {"role": "a", "name": "More information..."}
      ]
    }
  ]
}
```

### Executing JavaScript

```
Tool: evaluate_js
Input: {"url": "https://example.com", "expression": "document.title"}
```

Output:
```
Some(String("Example Domain"))
```

### Interacting with forms

```
Tool: type_text
Input: {
  "url": "https://www.google.com",
  "selector": "textarea[name='q']",
  "text": "rust programming"
}
```

Output:
```
Typed into 'textarea[name='q']'
```

### Lightweight fetch (no Chrome)

```
Tool: fetch_light
Input: {"url": "https://httpbin.org/html"}
```

Output:
```
Herman Melville - Moby Dick Availing himself of the mild, summer-cool
weather that now reigned in these latitudes, and in preparation for the
peculiarly active pursuits shortly to be anticipated...
```

## Performance

| Mode | Use case | Speed |
|------|----------|-------|
| `check_status` | URL validation | ~100ms |
| `fetch_light` | Static pages, APIs | ~200ms |
| `read_page` | JS-rendered pages | ~1-3s |
| `screenshot` | Visual capture | ~2-4s |
| `get_accessibility_tree` | Structured extraction | ~1-3s |

## Comparison with other browser MCPs

| Feature | mcp-browser | Playwright MCP | Puppeteer MCP |
|---------|:-----------:|:--------------:|:-------------:|
| Language | Rust | TypeScript | TypeScript |
| Chrome management | Auto | Manual | Manual |
| Startup time | ~500ms | ~2s | ~2s |
| Memory usage | Low | High | High |
| Accessibility tree | ✅ | ✅ | ❌ |
| Structured extraction | ✅ | ❌ | ❌ |
| Lightweight mode | ✅ | ❌ | ❌ |
| PDF export | ✅ | ✅ | ✅ |
| Form interaction | ✅ | ✅ | ✅ |

## Troubleshooting

### Chrome fails to launch

```bash
# Check if Chrome/Chromium is available
which chromium || which google-chrome

# On macOS, headless_chrome downloads Chromium automatically
# On Linux, you may need:
apt-get install -y chromium-browser  # Debian/Ubuntu
```

### Sandbox errors on Linux

If running in Docker or CI:
```bash
# The server already disables sandbox, but if issues persist:
export CHROME_FLAGS="--no-sandbox --disable-dev-shm-usage"
```

### Timeout on slow pages

The default navigation timeout is 30s. For very slow pages, use `fetch_light` which doesn't wait for JS rendering.

## Documentation

- [API Reference](docs/api-reference.md) — detailed tool parameters and response schemas
- [CHANGELOG](CHANGELOG.md) — version history

## License

Apache-2.0 — Part of [ADK-Rust Enterprise](https://www.zavora.ai)

## rmcp and MCP compatibility

This server is built with [`rmcp` 3.1.2](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.2) and requires Rust 1.88 or newer. The rmcp 3 rollout retains legacy MCP initialization compatibility and targets MCP protocol revisions `2025-11-25` and `2026-07-28`.
