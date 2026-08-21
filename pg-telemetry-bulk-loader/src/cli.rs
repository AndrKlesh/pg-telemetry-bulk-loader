// Модуль парсинга аргументов командной строки для консольного приложения
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::fmt;

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
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

// Вывод уровня логирования обычным текстом
impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            LogLevel::Critical => "critical",
            LogLevel::Error => "error",
            LogLevel::Warning => "warning",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        };
        write!(f, "{value}")
    }
}

// Функция
pub fn get_args() -> Result<Arguments,String> {
    let args = Arguments::parse(); // парсинг структуры аргументов
    // Проверка количества строк
    if args.batch_size == 0 || args.batch_size > 1000000 {
        return Err("Ошибка: Количество строк в буфере должно быть > 0 и ≤ 1 000 000 
        (batch-size должен быть от 1 до 1 000 000.)".to_string()); // eprintln!() - для того чтоббы вывело в поток ошибок
    }
    // Проверка каталога
    //if !args.input_dir.exists() { // если НЕ существует
    //    return Err("Ошибка: Указанный каталог не существует.".to_string());
    //}
    //if !args.input_dir.is_dir() { // если НЕ является папкой 
     //   return Err("Ошибка: Указанный путь не является каталогом.".to_string());
    //}
    // Проверка строки подключения
    if !args.db_connection.contains("host=") || !args.db_connection.contains("dbname=") || !args.db_connection.contains("user=") {
        return Err("Ошибка: неправильная строка подключения к PostgreSQL.".to_string());
    }
    Ok(args)
}