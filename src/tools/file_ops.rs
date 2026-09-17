use super::sandbox::{safe_path, MAX_FILE_SIZE};

/// Bir dosyayı okur (sandbox içinde, max 1 MB)
pub fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = safe_path(path)?;
    println!("[DEBUG] Dosya okunuyor: {:?}", full_path);

    // Dosya var mı?
    if !full_path.exists() {
        return Err(format!("Dosya bulunamadi: {}", path).into());
    }

    // Boyut kontrolü
    let metadata = std::fs::metadata(&full_path)?;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(format!(
            "Dosya cok buyuk: {} byte (max {} byte)",
            metadata.len(),
            MAX_FILE_SIZE
        )
        .into());
    }

    // İçeriği oku
    let content = std::fs::read_to_string(&full_path)?;
    println!("[DEBUG] Okunan boyut: {} karakter", content.len());

    Ok(content)
}

/// Bir dosyaya yazar (sandbox içinde, max 1 MB)
pub fn write_file(path: &str, content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let full_path = safe_path(path)?;
    println!("[DEBUG] Dosya yaziliyor: {:?}", full_path);

    // Boyut kontrolü
    if content.len() as u64 > MAX_FILE_SIZE {
        return Err(format!(
            "Icerik cok buyuk: {} byte (max {} byte)",
            content.len(),
            MAX_FILE_SIZE
        )
        .into());
    }

    // Parent dizinleri oluştur
    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Yaz
    std::fs::write(&full_path, content)?;
    println!("[DEBUG] Yazilan boyut: {} byte", content.len());

    Ok(format!(
        "Dosya basariyla yazildi: {} ({} byte)",
        path,
        content.len()
    ))
}

/// Bir dizini listeler (sandbox içinde)
pub fn list_dir(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dir_path = if path.trim().is_empty() || path == "." {
        // Workspace dizinini garanti et
        super::sandbox::ensure_workspace()?
    } else {
        safe_path(path)?
    };

    println!("[DEBUG] Dizin listeleniyor: {:?}", dir_path);

    if !dir_path.exists() {
        return Err(format!("Dizin bulunamadi: {}", path).into());
    }

    if !dir_path.is_dir() {
        return Err(format!("Bu bir dizin degil: {}", path).into());
    }

    let mut entries: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(&dir_path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            entries.push(format!("[DIR]  {}/", name));
        } else {
            entries.push(format!("[FILE] {} ({} byte)", name, metadata.len()));
        }
    }

    entries.sort();

    if entries.is_empty() {
        return Ok("(bos dizin)".to_string());
    }

    Ok(entries.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_file() {
        let content = "Merhaba, dunya!";
        let write_result = write_file("test_write_read.txt", content);
        assert!(write_result.is_ok());

        let read_result = read_file("test_write_read.txt");
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        // Temizlik
        let _ = std::fs::remove_file("workspace/test_write_read.txt");
    }

    #[test]
    fn test_write_file_with_subdir() {
        let result = write_file("subdir/test.txt", "content");
        assert!(result.is_ok());

        let read_result = read_file("subdir/test.txt");
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), "content");

        // Temizlik
        let _ = std::fs::remove_file("workspace/subdir/test.txt");
        let _ = std::fs::remove_dir("workspace/subdir");
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_file("nonexistent_file_xyz.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_rejects_path_traversal() {
        let result = write_file("../outside.txt", "content");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_rejects_absolute() {
        let result = write_file("/tmp/outside.txt", "content");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_rejects_too_large() {
        let big_content = "x".repeat(2_000_000); // 2 MB
        let result = write_file("big.txt", &big_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cok buyuk"));
    }

    #[test]
    fn test_list_dir_empty() {
        let result = list_dir(".");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_dir_nonexistent() {
        let result = list_dir("nonexistent_dir_xyz");
        assert!(result.is_err());
    }
}
