use std::path::{Path, PathBuf};

/// Sandbox dizini: tüm dosya işlemleri burada sınırlı
pub const WORKSPACE_DIR: &str = "workspace";

/// Maksimum dosya boyutu (1 MB)
pub const MAX_FILE_SIZE: u64 = 1_048_576;

/// Sandbox dizinini oluşturur (yoksa)
pub fn ensure_workspace() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let workspace = PathBuf::from(WORKSPACE_DIR);
    if !workspace.exists() {
        std::fs::create_dir_all(&workspace)?;
        println!("[DEBUG] Workspace dizini olusturuldu: {}", WORKSPACE_DIR);
    }
    Ok(workspace)
}

/// Verilen yolu sandbox içinde güvenli bir yola çevirir.
/// Path traversal (`..`) ve absolute path'leri reddeder.
pub fn safe_path(user_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Boş yol kontrolü
    if user_path.trim().is_empty() {
        return Err("Bos yol kabul edilmez.".into());
    }

    // Absolute path kontrolü (hem Unix hem Windows)
    let path = Path::new(user_path);
    if path.is_absolute() || user_path.starts_with('/') || user_path.starts_with('\\') {
        return Err("Absolute yollar kabul edilmez.".into());
    }

    // Windows drive letter kontrolü (C:, D:, vb.)
    if user_path.len() >= 2 {
        let bytes = user_path.as_bytes();
        if bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            return Err("Absolute yollar kabul edilmez.".into());
        }
    }

    // Path traversal kontrolü
    for component in path.components() {
        if let std::path::Component::ParentDir = component {
            return Err("Path traversal ('..') kabul edilmez.".into());
        }
    }

    // Sandbox içinde birleştir
    let workspace = ensure_workspace()?;
    let full_path = workspace.join(user_path);

    // Canonicalize ile son kontrol
    if let Some(parent) = full_path.parent() {
        if parent.exists() {
            let canonical_parent = parent.canonicalize()?;
            let canonical_workspace = workspace.canonicalize()?;
            if !canonical_parent.starts_with(&canonical_workspace) {
                return Err("Sandbox disina cikma girisimi engellendi.".into());
            }
        }
    }

    Ok(full_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_path_simple() {
        let result = safe_path("test.txt");
        assert!(result.is_ok());
        assert!(result.unwrap().to_string_lossy().contains("workspace"));
    }

    #[test]
    fn test_safe_path_with_subdir() {
        let result = safe_path("subdir/test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_safe_path_rejects_parent_dir() {
        let result = safe_path("../etc/passwd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Path traversal"));
    }

    #[test]
    fn test_safe_path_rejects_absolute() {
        let result = safe_path("/etc/passwd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Absolute"));
    }

    #[test]
    fn test_safe_path_rejects_empty() {
        let result = safe_path("");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_path_rejects_windows_absolute() {
        let result = safe_path("C:\\Windows\\System32");
        assert!(result.is_err());
    }
}
