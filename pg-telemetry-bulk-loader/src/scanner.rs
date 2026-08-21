use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// рекурсивно ищет CSV-файлы в каталоге
pub fn scan_directory(path: &Path) -> io::Result<Vec<PathBuf>> {
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound,"Каталог не существует",));
    }
    if !path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,"Указанный путь не является каталогом",));
    }
    let mut files = Vec::new();
    scan_recursive(path, &mut files)?;
    Ok(files)
}
// пекурсивный обход каталогов
fn scan_recursive(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Если это каталог, заходим внутрь
            scan_recursive(&path, files)?;
        } else if path.is_file() {
            // Если это файл, проверяем расширение
            if is_csv(&path) {
                files.push(path);
            }
        }
    }
    Ok(())
}
// проверяет, является ли файл CSV
fn is_csv(path: &Path) -> bool {
    match path.extension() {
        Some(extension) => {
            extension.to_string_lossy().eq_ignore_ascii_case("csv")
        }
        None => false,
    }
}
