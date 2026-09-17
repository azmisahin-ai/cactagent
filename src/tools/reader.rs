use scraper::{Html, Selector};
use std::io::Cursor;

pub fn read_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[DEBUG] URL okunuyor: {}", url);

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let body = client.get(url).send()?.text()?;
    println!("[DEBUG] Body uzunlugu: {} karakter", body.len());

    // Önce readability dene
    let extracted = match readability_extract(&body, url) {
        Ok(text) if !text.trim().is_empty() => {
            println!("[DEBUG] Readability basarili: {} karakter", text.len());
            text
        }
        Ok(_) => {
            println!("[DEBUG] Readability bos dondu, fallback kullaniliyor");
            extract_with_density(&body)?
        }
        Err(e) => {
            println!(
                "[DEBUG] Readability basarisiz: {}, fallback kullaniliyor",
                e
            );
            extract_with_density(&body)?
        }
    };

    let cleaned = clean_text(&extracted);
    let truncated = smart_truncate(&cleaned, 3000);

    Ok(truncated)
}

fn readability_extract(html: &str, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parsed_url: url::Url = url.parse()?;
    let mut cursor = Cursor::new(html.as_bytes());
    let article = readability::extractor::extract(&mut cursor, &parsed_url)?;
    let doc = Html::parse_fragment(&article.content);
    Ok(doc.root_element().text().collect::<Vec<_>>().join(" "))
}

fn extract_with_density(body: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(body);
    let remove_selector =
        Selector::parse("script, style, nav, header, footer, aside, form, iframe, svg, noscript")
            .unwrap();

    let mut clean_html = body.to_string();
    for element in document.select(&remove_selector) {
        clean_html = clean_html.replace(&element.html(), "");
    }

    let clean_doc = Html::parse_document(&clean_html);
    let content_selectors = [
        "article",
        "main",
        "[role=main]",
        ".content",
        ".post",
        ".entry-content",
        "body",
    ];

    let mut best_text = String::new();
    let mut best_length = 0;

    for selector_str in &content_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in clean_doc.select(&selector) {
                let text = extract_text_from_element(&element);
                if text.len() > best_length {
                    best_length = text.len();
                    best_text = text;
                }
            }
        }
    }

    Ok(best_text)
}

fn extract_text_from_element(element: &scraper::ElementRef) -> String {
    let mut text = String::new();

    let block_selector =
        Selector::parse("p, h1, h2, h3, h4, h5, h6, li, blockquote, pre, td").unwrap();

    for block in element.select(&block_selector) {
        let block_text: String = block.text().collect::<Vec<_>>().join(" ");
        let trimmed = block_text.trim();
        if !trimmed.is_empty() {
            text.push_str(trimmed);
            text.push('\n');
        }
    }

    if text.is_empty() {
        text = element.text().collect::<Vec<_>>().join(" ");
    }

    text
}

fn clean_text(text: &str) -> String {
    let mut cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");

    // Breadcrumb ve navigasyon kalıplarını temizle
    if cleaned.contains("» ") {
        if let Some(last_arrow) = cleaned.rfind("» ") {
            cleaned = cleaned[last_arrow + 3..].to_string();
        }
    }

    // "Ana Sayfa", "Home", "Menu" gibi kalıpları baştan temizle
    let noise_prefixes = [
        "Ana Sayfa ",
        "Home ",
        "Menu ",
        "Skip to content ",
        "Anasayfa ",
        "İçeriğe geç ",
    ];
    for prefix in &noise_prefixes {
        if cleaned.starts_with(prefix) {
            cleaned = cleaned[prefix.len()..].to_string();
        }
    }

    let lines: Vec<&str> = cleaned.lines().collect();
    let filtered: Vec<&str> = lines
        .iter()
        .filter(|line| {
            line.len() > 20 || line.ends_with('.') || line.ends_with('!') || line.ends_with('?')
        })
        .copied()
        .collect();

    cleaned = filtered.join("\n");

    while cleaned.contains("\n\n\n") {
        cleaned = cleaned.replace("\n\n\n", "\n\n");
    }

    cleaned.trim().to_string()
}

fn smart_truncate(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    let truncated: String = text.chars().take(max_chars).collect();

    let last_sentence_end = truncated
        .rfind(['.', '!', '?'])
        .map(|i| i + 1)
        .unwrap_or(truncated.len());

    if last_sentence_end < max_chars / 2 {
        truncated
    } else {
        truncated[..last_sentence_end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_truncate_short_text() {
        let text = "Short text.";
        let result = smart_truncate(text, 100);
        assert_eq!(result, "Short text.");
    }

    #[test]
    fn test_smart_truncate_long_text() {
        let text = "First sentence. Second sentence. Third sentence. Fourth sentence.";
        let result = smart_truncate(text, 30);
        // 30 karakterden kısa olmalı ve cümle sonunda bitmeli
        assert!(result.len() <= 30);
        assert!(result.ends_with('.'));
    }

    #[test]
    fn test_smart_truncate_no_sentence_end() {
        let text = "abcdefghijklmnopqrstuvwxyz";
        let result = smart_truncate(text, 10);
        assert_eq!(result, "abcdefghij");
    }

    #[test]
    fn test_clean_text_removes_breadcrumb() {
        let text = "Ana Sayfa » Kategori » Alt Kategori Bu makalenin gerçek içeriği burada başlar.";
        let result = clean_text(text);
        // Breadcrumb temizlenmeli
        assert!(!result.contains("»"));
    }

    #[test]
    fn test_clean_text_multiple_spaces() {
        let text = "Bu    bir    test    metnidir.";
        let result = clean_text(text);
        assert_eq!(result, "Bu bir test metnidir.");
    }

    #[test]
    fn test_clean_text_empty() {
        let result = clean_text("");
        assert_eq!(result, "");
    }
}
