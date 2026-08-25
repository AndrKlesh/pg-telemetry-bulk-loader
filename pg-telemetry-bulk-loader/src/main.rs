mod cli; //работа с модулями
mod logger;
mod parser;
mod scanner;
use crate::cli::get_args;
use crate::parser::parse_measurement;
use log::{debug, error, info, trace};

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
    // Здесь просто проверяю работу парсера.
    // Дальше здесь будет чтение строк.
    let test_line = r#"1001,1,23.5,"2026-07-15 14:30:00""#;
    match parse_measurement(test_line) {
        Ok(measurement) => {
            info!(
                target: "info",
                "Измерение успешно обработано: object_id={}, measure_type_id={}, value={}, timestamp={}", // использовал поля, которые раньше не использовались и выдавали ошибку
                measurement.object_id,
                measurement.measure_type_id,
                measurement.value,
                measurement.timestamp
            );
        }
        Err(error) => {
            error!(
                target: "error",
                "Ошибка парсинга: {}",
                error
            );
        }
    }
    info!(
        target: "info",
        "Сканирование завершено"
    );

    //println!("Каталог: {}", args.input_dir.display());
    //println!("Размер batch: {}", args.batch_size);
    //println!("Уровень логирования: {:?}", args.log_level); // {:?} - вывод через Debug, так проще посмотреть что лежит в LogLevel
}
