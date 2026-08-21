mod cli; //работа с модулями
mod logger;
mod parser;
mod scanner;
use crate::cli::get_args;
use crate::parser::parse_measurement;
use log::{debug, info, trace, warn}; //еще error, но просто не используется

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    // Настраиваем логирование
    logger::init(args.log_level);
    info!(target: "info","Приложение запущено");
    debug!(target: "debug","Каталог входных файлов: {}",args.input_dir.display());
    debug!(target: "debug","Размер batch: {}",args.batch_size);
    trace!(target: "trace","Аргументы командной строки: {:?}",args);

    // Сканируем каталог
    let files = match scanner::scan_directory(&args.input_dir) {
        Ok(files) => files,
        Err(error) => {
            log::error!(
                target: "critical",
                "Не удалось просканировать каталог: {}",
                error
            );
            return; //фатальная ошибка поэтому завершаем main?
        }
    };

    // Выводим количество найденных CSV-файлов
    info!(target: "info","Найдено CSV-файлов: {}",files.len());

    // Выводим полный список файлов при DEBUG
    for file in &files {
        debug!(target: "debug","Найден файл: {}",file.display());
    }

    // здесь просто проверяю работу парсера
    // дальше здесь будет чтение строк
    let test_line = r#"1001,1,23.5,"2026-07-15 14:30:00""#;
    match parse_measurement(test_line) {
        Some(measurement) => {
            info!(target: "info","Измерение успешно обработано: {:?}",measurement);
        }

        None => {
            // Ошибка уже записана parser.rs
            warn!(target: "warning","Строка измерения была пропущена");
        }
    }

    info!(target: "info","Сканирование завершено");

    //println!("Каталог: {}", args.input_dir.display());
    //println!("Размер batch: {}", args.batch_size);
    //println!("Уровень логирования: {:?}", args.log_level); // {:?} - вывод через Debug, так проще посмотреть что лежит в LogLevel
}
