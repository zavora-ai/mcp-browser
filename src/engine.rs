use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::sync::Arc;
use std::time::Duration;

pub struct BrowserEngine {
    browser: Browser,
}

impl BrowserEngine {
    pub fn launch() -> Result<Self> {
        let options = LaunchOptions::default_builder()
            .headless(true)
            .sandbox(false)
            .idle_browser_timeout(Duration::from_secs(300))
            .build()?;
        let browser = Browser::new(options)?;
        tracing::info!("Chrome launched (headless)");
        Ok(Self { browser })
    }

    pub fn new_tab(&self, url: &str) -> Result<Arc<Tab>> {
        let tab = self.browser.new_tab()?;
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;
        Ok(tab)
    }

    pub fn get_content(&self, url: &str) -> Result<String> {
        let tab = self.new_tab(url)?;
        let html = tab.get_content()?;
        tab.close(true)?;
        Ok(html)
    }

    pub fn screenshot_page(&self, url: &str) -> Result<Vec<u8>> {
        let tab = self.new_tab(url)?;
        let png = tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )?;
        tab.close(true)?;
        Ok(png)
    }

    pub fn screenshot_element(&self, url: &str, selector: &str) -> Result<Vec<u8>> {
        let tab = self.new_tab(url)?;
        let el = tab.find_element(selector)?;
        let png = el.capture_screenshot(headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png)?;
        tab.close(true)?;
        Ok(png)
    }

    pub fn save_pdf(&self, url: &str) -> Result<Vec<u8>> {
        let tab = self.new_tab(url)?;
        let pdf = tab.print_to_pdf(None)?;
        tab.close(true)?;
        Ok(pdf)
    }

    pub fn click(&self, url: &str, selector: &str) -> Result<String> {
        let tab = self.new_tab(url)?;
        tab.find_element(selector)?.click()?;
        std::thread::sleep(Duration::from_millis(500));
        let html = tab.get_content()?;
        tab.close(true)?;
        Ok(html)
    }

    pub fn type_text(&self, url: &str, selector: &str, text: &str) -> Result<()> {
        let tab = self.new_tab(url)?;
        tab.find_element(selector)?.click()?;
        tab.type_str(text)?;
        tab.close(true)?;
        Ok(())
    }

    pub fn evaluate_js(&self, url: &str, expression: &str) -> Result<String> {
        let tab = self.new_tab(url)?;
        let result = tab.evaluate(expression, false)?;
        tab.close(true)?;
        Ok(format!("{:?}", result.value))
    }

    pub fn get_accessibility_tree(&self, url: &str) -> Result<serde_json::Value> {
        let tab = self.new_tab(url)?;
        let tree = tab.evaluate(
            r#"(function() {
                function walk(el) {
                    const role = el.getAttribute('role') || el.tagName.toLowerCase();
                    const name = el.getAttribute('aria-label') || el.textContent?.trim().substring(0, 50) || '';
                    const children = Array.from(el.children).map(walk).filter(c => c);
                    const node = {role, name};
                    if (children.length) node.children = children;
                    return node;
                }
                return walk(document.body);
            })()"#,
            false,
        )?;
        tab.close(true)?;
        Ok(tree.value.unwrap_or(serde_json::json!(null)))
    }
}

/// Convert HTML to clean readable text (strip tags, extract content)
pub fn html_to_text(html: &str) -> String {
    use scraper::{Html, Selector};
    let doc = Html::parse_document(html);
    // Remove script/style
    let body_sel = Selector::parse("body").unwrap();
    if let Some(body) = doc.select(&body_sel).next() {
        let text: String = body.text().collect::<Vec<_>>().join(" ");
        // Clean up whitespace
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        String::new()
    }
}

/// Extract links from HTML
pub fn extract_links(html: &str, base_url: &str) -> Vec<serde_json::Value> {
    use scraper::{Html, Selector};
    let doc = Html::parse_document(html);
    let sel = Selector::parse("a[href]").unwrap();
    doc.select(&sel).filter_map(|el| {
        let href = el.value().attr("href")?;
        let text = el.text().collect::<String>().trim().to_string();
        let full_url = if href.starts_with("http") { href.to_string() } else { format!("{}{}", base_url.trim_end_matches('/'), href) };
        Some(serde_json::json!({"href": full_url, "text": text}))
    }).take(100).collect()
}

/// Extract metadata from HTML
pub fn extract_metadata(html: &str) -> serde_json::Value {
    use scraper::{Html, Selector};
    let doc = Html::parse_document(html);
    let title = Selector::parse("title").ok().and_then(|s| doc.select(&s).next()).map(|e| e.text().collect::<String>());
    let desc = Selector::parse("meta[name=description]").ok().and_then(|s| doc.select(&s).next()).and_then(|e| e.value().attr("content").map(String::from));
    let og_title = Selector::parse("meta[property='og:title']").ok().and_then(|s| doc.select(&s).next()).and_then(|e| e.value().attr("content").map(String::from));
    let og_image = Selector::parse("meta[property='og:image']").ok().and_then(|s| doc.select(&s).next()).and_then(|e| e.value().attr("content").map(String::from));
    let canonical = Selector::parse("link[rel=canonical]").ok().and_then(|s| doc.select(&s).next()).and_then(|e| e.value().attr("href").map(String::from));
    serde_json::json!({"title": title, "description": desc, "og_title": og_title, "og_image": og_image, "canonical": canonical})
}

/// Query elements by CSS selector
pub fn query_selector(html: &str, selector: &str) -> Vec<String> {
    use scraper::{Html, Selector};
    let doc = Html::parse_document(html);
    if let Ok(sel) = Selector::parse(selector) {
        doc.select(&sel).take(50).map(|el| el.text().collect::<String>().trim().to_string()).filter(|t| !t.is_empty()).collect()
    } else {
        vec![format!("Invalid selector: {}", selector)]
    }
}
