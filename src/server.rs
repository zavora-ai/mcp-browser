use crate::engine::{self, BrowserEngine};
use base64::Engine as _;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UrlInput { pub url: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SelectorInput { pub url: String, pub selector: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TypeInput { pub url: String, pub selector: String, pub text: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct JsInput { pub url: String, pub expression: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct WaitInput { pub url: String, pub selector: String, pub timeout_ms: Option<u64> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CookieInput { pub url: String, pub cookies: Vec<Cookie> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct Cookie { pub name: String, pub value: String, pub domain: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ExtractInput { pub url: String, pub fields: serde_json::Value }

#[derive(Clone)]
pub struct BrowserServer {
    pub engine: Arc<Mutex<BrowserEngine>>,
}

fn run_blocking<F, T>(f: F) -> T where F: FnOnce() -> T + Send + 'static, T: Send + 'static {
    std::thread::spawn(f).join().unwrap()
}

#[tool_router(server_handler)]
impl BrowserServer {
    // === Read & Extract (7) ===

    #[tool(description = "Fetch a URL and return clean readable text (markdown-friendly, no HTML noise)")]
    async fn read_page(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url.clone();
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_content(&url)
        }).await.unwrap() {
            Ok(html) => engine::html_to_text(&html),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Get full rendered HTML of a page (after JavaScript execution)")]
    async fn get_html(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_content(&url).unwrap_or_else(|e| format!("Error: {}", e))
        }).await.unwrap()
    }

    #[tool(description = "Extract all links from a page (href + text)")]
    async fn extract_links(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url.clone();
        let html = tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_content(&url).unwrap_or_default()
        }).await.unwrap();
        let links = engine::extract_links(&html, &input.url);
        serde_json::to_string_pretty(&links).unwrap()
    }

    #[tool(description = "Extract page metadata: title, description, OG tags, canonical URL")]
    async fn extract_metadata(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let html = tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_content(&url).unwrap_or_default()
        }).await.unwrap();
        serde_json::to_string_pretty(&engine::extract_metadata(&html)).unwrap()
    }

    #[tool(description = "Query elements matching a CSS selector — returns their text content")]
    async fn query_selector(&self, Parameters(input): Parameters<SelectorInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let selector = input.selector;
        let html = tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_content(&url).unwrap_or_default()
        }).await.unwrap();
        let results = engine::query_selector(&html, &selector);
        serde_json::to_string_pretty(&results).unwrap()
    }

    #[tool(description = "Get the accessibility tree of a page (structured, low-token representation)")]
    async fn get_accessibility_tree(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_accessibility_tree(&url)
        }).await.unwrap() {
            Ok(tree) => serde_json::to_string_pretty(&tree).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Extract structured data from a page using CSS selectors (e.g. {\"title\": \"h1\", \"price\": \".price\"})")]
    async fn extract_structured(&self, Parameters(input): Parameters<ExtractInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let fields = input.fields;
        let html = tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_content(&url).unwrap_or_default()
        }).await.unwrap();
        let mut result = json!({});
        if let Some(obj) = fields.as_object() {
            for (key, selector_val) in obj {
                if let Some(sel) = selector_val.as_str() {
                    let values = engine::query_selector(&html, sel);
                    result[key] = if values.len() == 1 { json!(values[0]) } else { json!(values) };
                }
            }
        }
        serde_json::to_string_pretty(&result).unwrap()
    }

    // === Visual (3) ===

    #[tool(description = "Take a full-page screenshot (returns base64 PNG)")]
    async fn screenshot(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.screenshot_page(&url)
        }).await.unwrap() {
            Ok(png) => {
                let b64 = base64::engine::general_purpose::STANDARD.encode(&png);
                json!({"format": "png", "size_bytes": png.len(), "base64": b64}).to_string()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Screenshot a specific element by CSS selector")]
    async fn screenshot_element(&self, Parameters(input): Parameters<SelectorInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let selector = input.selector;
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.screenshot_element(&url, &selector)
        }).await.unwrap() {
            Ok(png) => {
                let b64 = base64::engine::general_purpose::STANDARD.encode(&png);
                json!({"format": "png", "size_bytes": png.len(), "base64": b64}).to_string()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Save a page as PDF (returns base64)")]
    async fn save_pdf(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.save_pdf(&url)
        }).await.unwrap() {
            Ok(pdf) => {
                let b64 = base64::engine::general_purpose::STANDARD.encode(&pdf);
                json!({"format": "pdf", "size_bytes": pdf.len(), "base64": b64}).to_string()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Interact (4) ===

    #[tool(description = "Click an element by CSS selector")]
    async fn click(&self, Parameters(input): Parameters<SelectorInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let selector = input.selector;
        let sel_clone = selector.clone();
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.click(&url, &selector)
        }).await.unwrap() {
            Ok(html) => format!("Clicked '{}'. Page has {} chars after click.", sel_clone, html.len()),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Type text into an input field")]
    async fn type_text(&self, Parameters(input): Parameters<TypeInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let selector = input.selector.clone();
        let text = input.text;
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.type_text(&url, &selector, &text)
        }).await.unwrap() {
            Ok(_) => format!("Typed into '{}'", input.selector),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Execute JavaScript on a page and return the result")]
    async fn evaluate_js(&self, Parameters(input): Parameters<JsInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let expr = input.expression;
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.evaluate_js(&url, &expr)
        }).await.unwrap() {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Wait for an element to appear on the page")]
    async fn wait_for(&self, Parameters(input): Parameters<WaitInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url;
        let selector = input.selector;
        let _timeout = input.timeout_ms.unwrap_or(5000);
        let sel_clone = selector.clone();
        match tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            let tab = eng.new_tab(&url)?;
            tab.wait_for_element(&selector)?;
            tab.close(true)?;
            Ok::<_, anyhow::Error>(())
        }).await.unwrap() {
            Ok(_) => format!("Element '{}' found", sel_clone),
            Err(e) => format!("Error: {}", e),
        }
    }

    // === Navigate & Session (3) ===

    #[tool(description = "Navigate to URL and return page info (status, title, load time)")]
    async fn navigate(&self, Parameters(input): Parameters<UrlInput>) -> String {
        let engine = self.engine.clone();
        let url = input.url.clone();
        let start = std::time::Instant::now();
        let html = tokio::task::spawn_blocking(move || {
            let eng = engine.blocking_lock();
            eng.get_content(&url)
        }).await.unwrap();
        let elapsed = start.elapsed().as_millis();
        match html {
            Ok(h) => {
                let meta = engine::extract_metadata(&h);
                json!({"url": input.url, "title": meta["title"], "load_time_ms": elapsed, "size_bytes": h.len()}).to_string()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Check URL status without rendering (HTTP HEAD — fast)")]
    async fn check_status(&self, Parameters(input): Parameters<UrlInput>) -> String {
        match reqwest::Client::new().head(&input.url).send().await {
            Ok(resp) => json!({
                "url": input.url, "status": resp.status().as_u16(),
                "content_type": resp.headers().get("content-type").and_then(|v| v.to_str().ok()),
                "content_length": resp.headers().get("content-length").and_then(|v| v.to_str().ok()),
            }).to_string(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Fetch page with lightweight HTTP (no Chrome, fast, for simple pages)")]
    async fn fetch_light(&self, Parameters(input): Parameters<UrlInput>) -> String {
        match reqwest::get(&input.url).await {
            Ok(resp) => {
                let html = resp.text().await.unwrap_or_default();
                engine::html_to_text(&html)
            }
            Err(e) => format!("Error: {}", e),
        }
    }
}
