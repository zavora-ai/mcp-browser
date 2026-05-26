# Changelog

## [1.0.0] — 2026-05-25

### Added
- **Read & Extract (7 tools):** `read_page`, `get_html`, `extract_links`, `extract_metadata`, `query_selector`, `get_accessibility_tree`, `extract_structured`
- **Visual (3 tools):** `screenshot`, `screenshot_element`, `save_pdf`
- **Interact (4 tools):** `click`, `type_text`, `evaluate_js`, `wait_for`
- **Navigate (3 tools):** `navigate`, `check_status`, `fetch_light`
- Headless Chrome via `headless_chrome` crate (native CDP)
- Lightweight HTTP mode via `reqwest` for simple pages
- HTML → clean text conversion using `scraper`
- CSS selector-based extraction
- Accessibility tree via JS evaluation
- Structured data extraction with selector maps
- Zero-configuration Chrome management (auto-download)
- Registry-compatible `mcp-server.toml` manifest (not yet added)

### Architecture
- `BrowserEngine` — manages headless Chrome lifecycle, tab creation, navigation
- `server.rs` — 17 tool handlers via `#[tool_router(server_handler)]`
- Dual mode: Chrome CDP for full rendering, reqwest for lightweight fetches
- Thread-safe engine access via `Arc<Mutex<BrowserEngine>>`
