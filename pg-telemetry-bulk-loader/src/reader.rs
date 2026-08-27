use crate::load_batch;
use crate::parser;
use crate::parser::parse_measurement;
use log::{error, info};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// просто все перенес в отдельный модуль, код такой же
pub fn process_file(path: &PathBuf, batch: &mut Vec<parser::Measurement>, batch_size: usize) {
    info!(
        target: "info",
        "Начало обработки файла: {}",
        path.display()
    );
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            error!(
                target: "error",
                "Не удалось открыть файл {}: {}",
                path.display(),
                error
            );
            return;
        }
    };
    let reader = BufReader::new(file);
    for (line_number, line) in reader.lines().enumerate() {
        // Пропускаем заголовок.
        if line_number == 0 {
            continue;
        }
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                error!(
                    target: "error",
                    "Не удалось прочитать строку {} в файле {}: {}",
                    line_number + 1,
                    path.display(),
                    error
                );
                continue;
            }
        };
        match parse_measurement(&line) {
            Ok(measurement) => {
                batch.push(measurement);
            }
            Err(error) => {
                error!(
                    target: "error",
                    "Ошибка в строке {} файла {}: {}",
                    line_number + 1,
                    path.display(),
                    error
                );
            }
        }
        if batch.len() == batch_size {
            load_batch(batch);
            batch.clear();
        }
    }
    // Загружаем остаток.
    if !batch.is_empty() {
        load_batch(batch);
        batch.clear();
    }
    info!(
        target: "info",
        "Файл обработан: {}",
        path.display()
    );
}
