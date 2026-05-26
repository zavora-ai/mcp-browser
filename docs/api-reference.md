# API Reference — mcp-browser

All tools communicate via MCP (Model Context Protocol) over stdio using JSON-RPC 2.0.

## Read & Extract

### read_page

Fetch a URL using headless Chrome and return clean readable text. All HTML tags, scripts, and styles are stripped. Ideal for feeding page content to an LLM.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |

**Returns:** Plain text content of the page body.

**Example:**
```json
{"url": "https://example.com"}
```
→ `"Example Domain This domain is for use in illustrative examples..."`

---

### get_html

Get the full rendered HTML of a page after JavaScript execution. Useful when you need the raw DOM.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |

**Returns:** Complete HTML string (can be large).

---

### extract_links

Extract all anchor links from a page with their text and resolved href.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |

**Returns:** JSON array of `{"href": "...", "text": "..."}` objects (max 100).

---

### extract_metadata

Extract page metadata: title, meta description, Open Graph tags, and canonical URL.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |

**Returns:**
```json
{
  "title": "Page Title",
  "description": "Meta description",
  "og_title": "OG Title",
  "og_image": "https://...",
  "canonical": "https://..."
}
```

---

### query_selector

Query elements matching a CSS selector and return their text content.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |
| `selector` | string | ✅ | CSS selector (e.g. `h1`, `.price`, `#main p`) |

**Returns:** JSON array of text strings (max 50 elements).

---

### get_accessibility_tree

Get a structured accessibility tree of the page. This is a compact representation that uses ~77% fewer tokens than raw HTML while preserving semantic structure.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |

**Returns:** JSON tree with `role`, `name`, and `children` fields.

---

### extract_structured

Extract multiple named fields from a page using a map of CSS selectors. Perfect for scraping product pages, articles, or any structured content.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |
| `fields` | object | ✅ | Map of field names to CSS selectors |

**Example input:**
```json
{
  "url": "https://shop.com/product/123",
  "fields": {
    "title": "h1",
    "price": ".price",
    "description": ".product-description",
    "reviews": ".review-count"
  }
}
```

**Returns:** JSON object with field names as keys and extracted text as values. If a selector matches multiple elements, returns an array.

---

## Visual

### screenshot

Take a full-page screenshot.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to capture |

**Returns:**
```json
{
  "format": "png",
  "size_bytes": 45231,
  "base64": "iVBORw0KGgo..."
}
```

---

### screenshot_element

Screenshot a specific element identified by CSS selector.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to load |
| `selector` | string | ✅ | CSS selector of element to capture |

**Returns:** Same format as `screenshot`.

---

### save_pdf

Render a page as PDF.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to render |

**Returns:**
```json
{
  "format": "pdf",
  "size_bytes": 128456,
  "base64": "JVBERi0xLjQK..."
}
```

---

## Interact

### click

Click an element on the page. Returns the page HTML after the click (useful for SPAs where clicking changes content).

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to load |
| `selector` | string | ✅ | CSS selector of element to click |

**Returns:** `"Clicked '.btn-submit'. Page has 15234 chars after click."`

---

### type_text

Type text into an input field. Clicks the element first to focus it.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to load |
| `selector` | string | ✅ | CSS selector of input element |
| `text` | string | ✅ | Text to type |

**Returns:** `"Typed into 'input[name=email]'"`

---

### evaluate_js

Execute arbitrary JavaScript on the page and return the result.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to load |
| `expression` | string | ✅ | JavaScript expression to evaluate |

**Example expressions:**
- `document.title` — get page title
- `document.querySelectorAll('a').length` — count links
- `JSON.stringify(performance.timing)` — get performance data
- `window.scrollTo(0, document.body.scrollHeight)` — scroll to bottom

**Returns:** String representation of the JS return value.

---

### wait_for

Wait for an element to appear on the page (useful for dynamically loaded content).

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to load |
| `selector` | string | ✅ | CSS selector to wait for |
| `timeout_ms` | number | ❌ | Timeout in milliseconds (default: 5000) |

**Returns:** `"Element '.results-loaded' found"` or error on timeout.

---

## Navigate

### navigate

Navigate to a URL and return page info including load time.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to navigate to |

**Returns:**
```json
{
  "url": "https://www.rust-lang.org",
  "title": "Rust Programming Language",
  "load_time_ms": 1243,
  "size_bytes": 19847
}
```

---

### check_status

Quick HTTP HEAD request to check if a URL is reachable. Does not use Chrome — very fast.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to check |

**Returns:**
```json
{
  "url": "https://api.github.com",
  "status": 200,
  "content_type": "application/json; charset=utf-8",
  "content_length": "2048"
}
```

---

### fetch_light

Fetch a page using plain HTTP (no Chrome). Returns clean text. Use this for:
- Static HTML pages
- API responses
- Pages that don't need JavaScript
- When speed matters more than rendering fidelity

**Parameters:**
| Name | Type | Required | Description |
|------|------|:--------:|-------------|
| `url` | string | ✅ | URL to fetch |

**Returns:** Plain text content (same format as `read_page` but without JS rendering).

---

## Error Handling

All tools return errors as plain text strings prefixed with `"Error: "`:

```
Error: net::ERR_NAME_NOT_RESOLVED
Error: Timeout waiting for element
Error: Invalid selector: !!!
```

## Rate Limiting

The server creates a new Chrome tab for each request and closes it after. There is no built-in rate limiting — the bottleneck is Chrome's tab creation speed (~100ms per tab). For high-throughput scraping, consider using `fetch_light` where possible.
