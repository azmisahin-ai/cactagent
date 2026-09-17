use scraper::{Html, Selector};

pub fn read_url(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("[DEBUG] URL okunuyor: {}", url);

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client.get(url).send()?;
    println!("[DEBUG] Yanit status: {}", response.status());

    let body = response.text()?;
    println!("[DEBUG] Body uzunlugu: {} karakter", body.len());

    let document = Html::parse_document(&body);

    // ADIM 1: Gürültülü etiketleri kaldır
    let remove_selector = Selector::parse(
        "script, style, nav, header, footer, aside, form, iframe, svg, noscript, \
         .sidebar, .menu, .nav, .footer, .header, .advertisement, .ad, .ads, \
         .cookie, .popup, .modal, .comments, .social, .share"
    ).unwrap();

    let mut clean_html = body.clone();
    for element in document.select(&remove_selector) {
        let element_html = element.html();
        clean_html = clean_html.replace(&element_html, "");
    }

    let clean_doc = Html::parse_document(&clean_html);

    // ADIM 2: İçeriği öncelikli etiketlerden çıkar
    // Önce <article>, sonra <main>, sonra <body> dene
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

    // Eğer hiçbir şey bulunamadıysa, tüm dokümanı dene
    if best_text.is_empty() {
        best_text = clean_doc.root_element().text().collect::<Vec<_>>().join(" ");
    }

    // ADIM 3: Metni temizle
    let cleaned = clean_text(&best_text);

    // ADIM 4: Akıllıca kısalt (cümle sonunda kes)
    let truncated = smart_truncate(&cleaned, 3000);

    if truncated.is_empty() {
        Ok("Sayfa icerigi okunamadi.".to_string())
    } else {
        println!("[DEBUG] Cikarilan metin uzunlugu: {} karakter", truncated.len());
        Ok(truncated)
    }
}

// Bir elementin içindeki metni, paragrafları koruyarak çıkarır.
fn extract_text_from_element(element: &scraper::ElementRef) -> String {
    let mut text = String::new();

    // Paragraf, başlık, liste öğelerini ayrı ayrı işle
    let block_selector = Selector::parse("p, h1, h2, h3, h4, h5, h6, li, blockquote, pre, td").unwrap();

    for block in element.select(&block_selector) {
        let block_text: String = block.text().collect::<Vec<_>>().join(" ");
        let trimmed = block_text.trim();
        if !trimmed.is_empty() {
            text.push_str(trimmed);
            text.push('\n');
        }
    }

    // Eğer blok bulunamadıysa, doğrudan metni al
    if text.is_empty() {
        text = element.text().collect::<Vec<_>>().join(" ");
    }

    text
}

// Metni temizler: fazla boşlukları, tekrarları, anlamsız karakterleri kaldırır.
fn clean_text(text: &str) -> String {
    // Fazla boşlukları tek boşluğa indir
    let mut cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");

    // Çok kısa satırları (muhtemelen menü öğeleri) kaldır
    let lines: Vec<&str> = cleaned.lines().collect();
    let filtered: Vec<&str> = lines
        .iter()
        .filter(|line| line.len() > 20 || line.ends_with('.') || line.ends_with('!') || line.ends_with('?'))
        .copied()
        .collect();

    cleaned = filtered.join("\n");

    // Tekrar eden boş satırları temizle
    while cleaned.contains("\n\n\n") {
        cleaned = cleaned.replace("\n\n\n", "\n\n");
    }

    cleaned.trim().to_string()
}

// Metni cümle sonunda keser (ortada kesmez).
fn smart_truncate(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    // max_chars'a kadar olan kısmı al
    let truncated: String = text.chars().take(max_chars).collect();

    // Son cümle sonunu bul (. ! ?)
    let last_sentence_end = truncated
        .rfind(|c| c == '.' || c == '!' || c == '?')
        .map(|i| i + 1)
        .unwrap_or(truncated.len());

    // Eğer cümle sonu çok erken bir yerdeyse (max_chars'ın %50'sinden az), olduğu gibi kes
    if last_sentence_end < max_chars / 2 {
        truncated
    } else {
        truncated[..last_sentence_end].to_string()
    }
}