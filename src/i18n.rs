//! Çoklu dil desteği modülü.
//!
//! Basit dil algılama ve yerelleştirme sağlar.
//! Hiçbir dış bağımlılık yok.

/// Desteklenen diller
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Turkish,
    English,
    German,
    French,
}

impl Language {
    /// Dil kodunu döndürür (ISO 639-1)
    pub fn code(&self) -> &'static str {
        match self {
            Language::Turkish => "tr",
            Language::English => "en",
            Language::German => "de",
            Language::French => "fr",
        }
    }

    /// DuckDuckGo bölge kodu
    pub fn ddg_region(&self) -> &'static str {
        match self {
            Language::Turkish => "tr-tr",
            Language::English => "us-en",
            Language::German => "de-de",
            Language::French => "fr-fr",
        }
    }

    /// Metinden dil koduna göre dil döndürür
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "tr" | "turkish" | "türkçe" => Some(Language::Turkish),
            "en" | "english" => Some(Language::English),
            "de" | "german" | "deutsch" => Some(Language::German),
            "fr" | "french" | "français" => Some(Language::French),
            _ => None,
        }
    }
}

/// Metinden dil algılar.
/// Basit karakter analizi kullanır (dış bağımlılık yok).
pub fn detect_language(text: &str) -> Language {
    let turkish_chars = ['ç', 'ğ', 'ı', 'İ', 'ö', 'ş', 'ü', 'Ç', 'Ğ', 'Ö', 'Ş', 'Ü'];
    let turkish_words = [
        "ve",
        "bir",
        "için",
        "ile",
        "bu",
        "olarak",
        "hakkında",
        "araştır",
    ];

    let german_chars = ['ä', 'ö', 'ü', 'ß'];
    let german_words = ["und", "der", "die", "das", "ist", "für", "mit"];

    let french_chars = ['é', 'è', 'ê', 'à', 'ç', 'ô', 'û'];
    let french_words = ["et", "le", "la", "les", "est", "pour", "avec"];

    let lower = text.to_lowercase();

    // Türkçe kontrolü
    let tr_char_count = text.chars().filter(|c| turkish_chars.contains(c)).count();
    let tr_word_count = turkish_words.iter().filter(|w| lower.contains(*w)).count();

    // Almanca kontrolü
    let de_char_count = text.chars().filter(|c| german_chars.contains(c)).count();
    let de_word_count = german_words.iter().filter(|w| lower.contains(*w)).count();

    // Fransızca kontrolü
    let fr_char_count = text.chars().filter(|c| french_chars.contains(c)).count();
    let fr_word_count = french_words.iter().filter(|w| lower.contains(*w)).count();

    let tr_score = tr_char_count * 3 + tr_word_count * 2;
    let de_score = de_char_count * 3 + de_word_count * 2;
    let fr_score = fr_char_count * 3 + fr_word_count * 2;

    // En yüksek puanı al
    if tr_score > 0 && tr_score >= de_score && tr_score >= fr_score {
        Language::Turkish
    } else if de_score > 0 && de_score >= fr_score {
        Language::German
    } else if fr_score > 0 {
        Language::French
    } else {
        Language::English
    }
}

/// Hata mesajlarını dile göre döndürür
pub fn error_message(lang: Language, key: &str) -> &'static str {
    match (lang, key) {
        // Genel hatalar
        (Language::Turkish, "search_failed") => "Arama yapilamadi.",
        (Language::Turkish, "rate_limit") => "Cok fazla istek. Lutfen bekleyin.",
        (Language::Turkish, "read_failed") => "Sayfa okunamadi.",
        (Language::Turkish, "file_not_found") => "Dosya bulunamadi.",
        (Language::Turkish, "permission_denied") => "Izin reddedildi.",
        (Language::Turkish, "unknown_tool") => "Bilinmeyen arac.",
        (Language::Turkish, "invalid_path") => "Gecersiz yol.",
        (Language::Turkish, "file_too_large") => "Dosya cok buyuk.",
        (Language::Turkish, "approved") => "Onaylandi.",
        (Language::Turkish, "rejected") => "Reddedildi.",

        // İngilizce (varsayılan)
        (_, "search_failed") => "Search failed.",
        (_, "rate_limit") => "Too many requests. Please wait.",
        (_, "read_failed") => "Failed to read page.",
        (_, "file_not_found") => "File not found.",
        (_, "permission_denied") => "Permission denied.",
        (_, "unknown_tool") => "Unknown tool.",
        (_, "invalid_path") => "Invalid path.",
        (_, "file_too_large") => "File too large.",
        (_, "approved") => "Approved.",
        (_, "rejected") => "Rejected.",
        (_, _) => "Unknown error.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_turkish() {
        assert_eq!(
            detect_language("Rust hakkında haberleri araştır"),
            Language::Turkish
        );
        assert_eq!(
            detect_language("Türkçe karakterler: çğıöşü"),
            Language::Turkish
        );
    }

    #[test]
    fn test_detect_english() {
        assert_eq!(
            detect_language("Search the web for Rust news"),
            Language::English
        );
        assert_eq!(detect_language("Hello world"), Language::English);
    }

    #[test]
    fn test_detect_german() {
        assert_eq!(
            detect_language("Suche nach Rust Nachrichten für"),
            Language::German
        );
    }

    #[test]
    fn test_detect_french() {
        assert_eq!(
            detect_language("Rechercher les nouvelles pour Rust"),
            Language::French
        );
    }

    #[test]
    fn test_language_code() {
        assert_eq!(Language::Turkish.code(), "tr");
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::German.code(), "de");
        assert_eq!(Language::French.code(), "fr");
    }

    #[test]
    fn test_ddg_region() {
        assert_eq!(Language::Turkish.ddg_region(), "tr-tr");
        assert_eq!(Language::English.ddg_region(), "us-en");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(Language::from_code("tr"), Some(Language::Turkish));
        assert_eq!(Language::from_code("TR"), Some(Language::Turkish));
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("xyz"), None);
    }

    #[test]
    fn test_error_message_turkish() {
        let msg = error_message(Language::Turkish, "search_failed");
        assert_eq!(msg, "Arama yapilamadi.");
    }

    #[test]
    fn test_error_message_english_fallback() {
        let msg = error_message(Language::German, "search_failed");
        assert_eq!(msg, "Search failed."); // İngilizce'ye düşer
    }
}
