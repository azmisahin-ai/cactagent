use scraper::{Html, Selector};

pub fn web_search(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        urlencoding::encode(query)
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(15))
        .build()?;

    let mut last_error: Option<Box<dyn std::error::Error>> = None;
    let mut body = String::new();

    for attempt in 1..=3 {
        println!("[DEBUG] Istek gonderiliyor (deneme {}/3): {}", attempt, url);

        match client.get(&url).send() {
            Ok(response) => {
                println!("[DEBUG] Yanit status: {}", response.status());
                body = response.text()?;
                println!("[DEBUG] Body uzunlugu: {} karakter", body.len());
                last_error = None;
                break;
            }
            Err(e) => {
                println!("[DEBUG] Deneme {} basarisiz: {}", attempt, e);
                last_error = Some(Box::new(e));
                if attempt < 3 {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }
    }

    if let Some(e) = last_error {
        return Err(e);
    }

    let document = Html::parse_document(&body);

    let result_selector = Selector::parse(".result").unwrap();
    let title_selector = Selector::parse(".result__a").unwrap();
    let snippet_selector = Selector::parse(".result__snippet").unwrap();

    let mut output = String::new();
    let mut count = 0;

    for result in document.select(&result_selector) {
        if count >= 5 {
            break;
        }

        let title = result
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let href = result
            .select(&title_selector)
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let clean_url = clean_ddg_url(&href);

        let snippet = result
            .select(&snippet_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        output.push_str(&format!(
            "{}. {}\n   URL: {}\n   {}\n\n",
            count + 1,
            title,
            clean_url,
            snippet
        ));
        count += 1;
    }

    if output.is_empty() {
        output = "Arama sonucu bulunamadi.".to_string();
    }

    Ok(output)
}

fn clean_ddg_url(raw_url: &str) -> String {
    if let Some(uddg_start) = raw_url.find("uddg=") {
        let after_uddg = &raw_url[uddg_start + 5..];
        let encoded = after_uddg.split('&').next().unwrap_or(after_uddg);
        if let Ok(decoded) = urlencoding::decode(encoded) {
            return decoded.to_string();
        }
    }
    raw_url.to_string()
}