//! Rate limiting modülü.
//!
//! Her aracın çok sık çağrılmasını engeller. Bu:
//! 1. DuckDuckGo gibi servislerin bizi engellemesini önler
//! 2. Ajanın kontrolsüz döngüye girmesini engeller
//! 3. Kendi kendine çalışan sistemlerde kaynak koruması sağlar

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Bir aracın rate limit yapılandırması
#[derive(Clone, Copy)]
pub struct RateLimit {
    /// Pencere süresi (saniye)
    pub window_secs: u64,
    /// Pencere içinde maksimum çağrı sayısı
    pub max_calls: usize,
}

impl RateLimit {
    pub const fn new(window_secs: u64, max_calls: usize) -> Self {
        Self {
            window_secs,
            max_calls,
        }
    }
}

/// Global rate limiter state
static RATE_LIMITER: Mutex<Option<HashMap<String, Vec<Instant>>>> = Mutex::new(None);

/// Bir aracın çağrılıp çağrılamayacağını kontrol eder.
/// Çağrılabilirse `Ok(())`, çağrılamazsa `Err(bekleme_saniyesi)` döner.
pub fn check_rate_limit(tool: &str, limit: RateLimit) -> Result<(), u64> {
    let mut guard = RATE_LIMITER.lock().unwrap();
    let map = guard.get_or_insert_with(HashMap::new);

    let now = Instant::now();
    let window = Duration::from_secs(limit.window_secs);

    // Bu aracın çağrı geçmişini al (yoksa oluştur)
    let calls = map.entry(tool.to_string()).or_default();

    // Pencere dışındaki çağrıları temizle
    calls.retain(|&t| now.duration_since(t) < window);

    // Limit kontrolü
    if calls.len() >= limit.max_calls {
        // En eski çağrının pencereden çıkması için ne kadar beklemeli?
        let oldest = calls[0];
        let elapsed = now.duration_since(oldest);
        let wait = window.saturating_sub(elapsed);
        return Err(wait.as_secs().max(1));
    }

    // Çağrıyı kaydet
    calls.push(now);
    Ok(())
}

/// Bir aracın mevcut çağrı sayısını döndürür (test/debug için).
#[allow(dead_code)]
pub fn call_count(tool: &str) -> usize {
    let guard = RATE_LIMITER.lock().unwrap();
    guard
        .as_ref()
        .and_then(|m| m.get(tool))
        .map(|v| v.len())
        .unwrap_or(0)
}

/// Rate limiter'ı sıfırlar (test için).
#[allow(dead_code)]
pub fn reset() {
    let mut guard = RATE_LIMITER.lock().unwrap();
    *guard = None;
}

/// Araç adına göre rate limit döndürür.
pub fn limit_for_tool(tool: &str) -> RateLimit {
    match tool {
        "web_search" => RateLimit::new(60, 10), // Dakikada 10
        "read_url" => RateLimit::new(60, 20),   // Dakikada 20
        "write_file" => RateLimit::new(60, 5),  // Dakikada 5
        "read_file" => RateLimit::new(60, 60),  // Dakikada 60
        "list_dir" => RateLimit::new(60, 60),   // Dakikada 60
        _ => RateLimit::new(60, 30),            // Varsayılan: dakikada 30
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_allows_first_call() {
        reset();
        let limit = RateLimit::new(60, 3);
        assert!(check_rate_limit("test_tool_1", limit).is_ok());
    }

    #[test]
    fn test_rate_limit_blocks_after_max() {
        reset();
        let limit = RateLimit::new(60, 3);

        // 3 çağrı geçmeli
        assert!(check_rate_limit("test_tool_2", limit).is_ok());
        assert!(check_rate_limit("test_tool_2", limit).is_ok());
        assert!(check_rate_limit("test_tool_2", limit).is_ok());

        // 4. çağrı engellenmeli
        let result = check_rate_limit("test_tool_2", limit);
        assert!(result.is_err());
        assert!(result.unwrap_err() > 0);
    }

    #[test]
    fn test_limit_for_tool() {
        let limit = limit_for_tool("web_search");
        assert_eq!(limit.max_calls, 10);
        assert_eq!(limit.window_secs, 60);
    }

    #[test]
    fn test_call_count() {
        reset();
        let limit = RateLimit::new(60, 10);
        check_rate_limit("test_tool_3", limit).ok();
        check_rate_limit("test_tool_3", limit).ok();
        assert_eq!(call_count("test_tool_3"), 2);
    }
}
