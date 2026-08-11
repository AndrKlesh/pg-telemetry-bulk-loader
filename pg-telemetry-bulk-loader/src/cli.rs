// Модуль парсинга аргументов командной строки для консольного приложения
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pg-telemetry-bulk-loader")]
// Структура всех аргументов
pub struct Arguments { // public - структура будет доступна для всех модулей
    // Путь к каталогу с CSV-файлами для загрузки
    #[arg(long)] // говорит о том что следующая строка будет аргументом, а long - указывает на длину имени
    pub input_dir: PathBuf, // PathBuf - специальный тип для хранения пути

    // Строка подключения к PostgreSQL
    #[arg(long)]
    pub db_connection: String,

    // Количество строк в буфере
    #[arg(long, default_value_t = 10000)] // стандартное значение
    pub batch_size: usize, // только положительные значения

    // Уровень логирования
    #[arg(long, value_enum, default_value_t = LogLevel::Info)] // value_enum для того чтобы clap использовал enum как пул значений аргумента
    // по стандарту LogLevel::Info чтоб показывало значение Info если не указать --log-level
    pub log_level: LogLevel,
}

// Пул значений доступных для --log-level
#[derive(Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

// Функция
pub fn get_args() -> Arguments {
    let args = Arguments::parse(); // парсинг структуры аргументов
    // Проверка количества строк
    if args.batch_size == 0 || args.batch_size > 1000000 {
        eprintln!("Ошибка: Количество строк в буфере должно быть > 0 и ≤ 1 000 000 
        (batch-size должен быть от 1 до 1 000 000.)"); // eprintln!() - для того чтоббы вывело в поток ошибок
        std::process::exit(1); // завершаем программу с кодом 1 (ошибка)
    }
}