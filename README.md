# Browser MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-browser.svg)](https://crates.io/crates/mcp-browser)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)

Web browsing for AI agents — read pages as clean text, take screenshots, extract data, interact with forms, and get accessibility trees. 17 tools powered by headless Chrome via native CDP.

## Tools (17)

### Read & Extract (7)

| Tool | Purpose |
|------|---------|
| `read_page` | Fetch URL → clean readable text (no HTML noise) |
| `get_html` | Full rendered HTML (after JS execution) |
| `extract_links` | All links with text and href |
| `extract_metadata` | Title, description, OG tags, canonical |
| `query_selector` | Extract elements by CSS selector |
| `get_accessibility_tree` | Structured page representation (77% less tokens) |
| `extract_structured` | Extract fields by selector map (e.g. `{"title": "h1", "price": ".price"}`) |

### Visual (3)

| Tool | Purpose |
|------|---------|
| `screenshot` | Full page screenshot (base64 PNG) |
| `screenshot_element` | Screenshot specific element |
| `save_pdf` | Save page as PDF |

### Interact (4)

| Tool | Purpose |
|------|---------|
| `click` | Click element by selector |
| `type_text` | Type into input field |
| `evaluate_js` | Run JavaScript, return result |
| `wait_for` | Wait for element to appear |

### Navigate (3)

| Tool | Purpose |
|------|---------|
| `navigate` | Go to URL, return title + load time |
| `check_status` | HTTP status check (fast, no Chrome) |
| `fetch_light` | Lightweight fetch (no Chrome, for simple pages) |

## Installation

```bash
cargo install mcp-browser
```

## Configuration

**Zero configuration.** Chrome is auto-downloaded and launched on first run.

```json
{ "mcpServers": { "browser": { "command": "mcp-browser" } } }
```

## Usage Examples

```
"Read the pricing page of competitor X"
→ read_page(url="https://competitor.com/pricing")

"Screenshot our landing page"
→ screenshot(url="https://oursite.com")

"Get all product prices"
→ extract_structured(url="https://shop.com", fields={"name": ".product-name", "price": ".price"})

"What's the page structure?"
→ get_accessibility_tree(url="https://app.com")
```

## How It Works

```
MCP Client → mcp-browser → headless_chrome (CDP) → Chromium
                         → reqwest (lightweight mode, no Chrome)
```

## License

Apache-2.0 — Part of [ADK-Rust Enterprise](https://enterprise.adk-rust.com)
