//! Audit log modülü.
//!
//! Her araç çağrısını `workspace/logs/audit.log` dosyasına yazar.
//! Bu, kendi kendine çalışan ajanın ne yaptığını izlemek için kritiktir.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log dizini
pub const LOG_DIR: &str = "workspace/logs";

/// Log dosyası
pub const LOG_FILE: &str = "workspace/logs/audit.log";

/// Log girdisini dosyaya yazar.
pub fn log_tool_call(tool: &str, args: &str, result: &str) {
    // Log dizinini oluştur
    if let Err(e) = fs::create_dir_all(LOG_DIR) {
        eprintln!("[AUDIT] Log dizini olusturulamadi: {}", e);
        return;
    }

    // Zaman damgası
    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_secs(),
        Err(_) => 0,
    };

    // İnsan okunabilir tarih
    let datetime = format_timestamp(timestamp);

    // Sonucu kısalt (log şişmesin)
    let result_preview: String = result.chars().take(100).collect();
    let result_display = if result.len() > 100 {
        format!("{}...", result_preview)
    } else {
        result_preview
    };

    // Log satırı
    let log_line = format!("[{}] {}({}) -> {}\n", datetime, tool, args, result_display);

    // Dosyaya ekle
    match OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(log_line.as_bytes()) {
                eprintln!("[AUDIT] Log yazilamadi: {}", e);
            }
        }
        Err(e) => {
            eprintln!("[AUDIT] Log dosyasi acilamadi: {}", e);
        }
    }
}

/// Unix timestamp'i okunabilir formata çevirir.
/// Format: "2026-09-17 23:45:12"
fn format_timestamp(secs: u64) -> String {
    // Basit tarih hesaplama (yerel saat dilimi kullanmadan)
    let days_since_epoch = secs / 86400;
    let secs_in_day = secs % 86400;

    let hours = secs_in_day / 3600;
    let minutes = (secs_in_day % 3600) / 60;
    let seconds = secs_in_day % 60;

    // 1970-01-01'den itibaren gün sayısı
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let month_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for days in month_days.iter() {
        if remaining_days < *days {
            break;
        }
        remaining_days -= days;
        month += 1;
    }

    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// Log dosyasını okur (son N satır).
#[allow(dead_code)]
pub fn read_recent_logs(n: usize) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(LOG_FILE)?;
    let lines: Vec<&str> = content.lines().collect();
    let start = if lines.len() > n { lines.len() - n } else { 0 };
    Ok(lines[start..].join("\n"))
}

/// Log dosyasını temizler.
#[allow(dead_code)]
pub fn clear_logs() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(LOG_FILE);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_format_timestamp() {
        // 2024-01-01 00:00:00 UTC = 1704067200
        let result = format_timestamp(1704067200);
        assert!(result.starts_with("2024-01-01"));
    }

    #[test]
    fn test_log_tool_call() {
        let _ = clear_logs();
        log_tool_call("test_tool", "test_arg", "test_result");
        // Log dosyası oluşmalı
        assert!(std::path::Path::new(LOG_FILE).exists());
        let _ = clear_logs();
    }
}
