use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// Ищет CSV-файлы в каталоге или возвращает указанный файл.
pub fn scan_directory(path: &Path) -> io::Result<Vec<PathBuf>> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Указанный путь не существует",
        ));
    }
    // Если передали конкретный файл,
    // просто добавляем его в результат.
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }
    // Если передали каталог, ищем CSV-файлы рекурсивно.
    if path.is_dir() {
        let mut files = Vec::new();
        scan_recursive(path, &mut files)?;
        return Ok(files);
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Указанный путь не является файлом или каталогом",
    ))
}

// Рекурсивный обход каталогов.
fn scan_recursive(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Если это каталог, заходим внутрь.
            scan_recursive(&path, files)?;
        } else if path.is_file() {
            // Если это файл, проверяем расширение.
            if is_csv(&path) {
                files.push(path);
            }
        }
    }
    Ok(())
}

// Проверяет, является ли файл CSV.
fn is_csv(path: &Path) -> bool {
    match path.extension() {
        Some(extension) => extension.to_string_lossy().eq_ignore_ascii_case("csv"),
        None => false,
    }
}
