mod cli; //работа с модулями
mod logger;
mod parser;
mod reader;
mod scanner;
use crate::cli::get_args;
use log::{debug, info, trace};

pub fn load_batch(batch: &[parser::Measurement]) {
    // сделал load_batch публичной
    info!(
        target: "info",
        "Загрузка batch: {} измерений",
        batch.len()
    );
    if let Some(first) = batch.first() {
        // потом это нужно будет убрать, либо что-то с этим сделать
        debug!(
            target: "debug",
            "Первое измерение batch: object_id={}, measure_type_id={}, value={}, timestamp={}",
            first.object_id,
            first.measure_type_id,
            first.value,
            first.timestamp
        );
    }
}

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };
    // Настраиваем логирование.
    logger::init(args.log_level);
    info!(target: "info", "Приложение запущено");
    debug!(
        target: "debug",
        "Каталог входных файлов: {}",
        args.input_dir.display()
    );
    debug!(
        target: "debug",
        "Размер batch: {}",
        args.batch_size
    );
    trace!(
        target: "trace",
        "Аргументы командной строки: {:?}",
        args
    );
    // Сканируем каталог или файл.
    let files = match scanner::scan_directory(&args.input_dir) {
        Ok(files) => files,
        Err(error) => {
            log::error!(
                target: "critical",
                "Не удалось просканировать путь: {}",
                error
            );
            return;
        }
    };
    // Выводим количество найденных CSV-файлов.
    info!(
        target: "info",
        "Найдено CSV-файлов: {}",
        files.len()
    );
    // Выводим полный список файлов при DEBUG.
    for file in &files {
        debug!(
            target: "debug",
            "Найден файл: {}",
            file.display()
        );
    }
    // Создаём пустой буфер с заранее выделенной ёмкостью.
    // думал сделать через vec![Measurement; batch_size], НО как я понял отличие в том что этот вариант создаст уже заполненный,
    // поскольку нужен пустой, я нашел вот такое решение:
    let mut batch = Vec::with_capacity(args.batch_size);
    // Обрабатываем каждый CSV-файл.
    for file in &files {
        reader::process_file(file, &mut batch, args.batch_size);
    }
    info!(
        target: "info",
        "Сканирование завершено"
    );
}
