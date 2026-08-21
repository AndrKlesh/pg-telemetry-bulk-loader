use crate::cli::LogLevel;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init(log_level: LogLevel) {
    let mut builder = Builder::new();
    // У crate `log` есть уровни: Error -> Warn -> Info -> Debug -> Trace, но отдельного уровня Critical в crate `log` нет
    // поэтму Critical и Error оба записываются через log::error!, но различаются target, чтоб env_logger мог фильтровать их отдельно
    match log_level {
        LogLevel::Critical => {
            builder
                .filter_level(LevelFilter::Off)
                .filter_module("critical", LevelFilter::Error);
        }
        LogLevel::Error => {
            builder
                .filter_level(LevelFilter::Off)
                .filter_module("critical", LevelFilter::Error)
                .filter_module("error", LevelFilter::Error);
        }
        LogLevel::Warning => {
            builder
                .filter_level(LevelFilter::Off)
                .filter_module("critical", LevelFilter::Error)
                .filter_module("error", LevelFilter::Error)
                .filter_module("warning", LevelFilter::Warn);
        }
        LogLevel::Info => {
            builder
                .filter_level(LevelFilter::Off)
                .filter_module("critical", LevelFilter::Error)
                .filter_module("error", LevelFilter::Error)
                .filter_module("warning", LevelFilter::Warn)
                .filter_module("info", LevelFilter::Info);
        }
        LogLevel::Debug => {
            builder
                .filter_level(LevelFilter::Off)
                .filter_module("critical", LevelFilter::Error)
                .filter_module("error", LevelFilter::Error)
                .filter_module("warning", LevelFilter::Warn)
                .filter_module("info", LevelFilter::Info)
                .filter_module("debug", LevelFilter::Debug);
        }

        LogLevel::Trace => {
            builder
                .filter_level(LevelFilter::Off)
                .filter_module("critical", LevelFilter::Error)
                .filter_module("error", LevelFilter::Error)
                .filter_module("warning", LevelFilter::Warn)
                .filter_module("info", LevelFilter::Info)
                .filter_module("debug", LevelFilter::Debug)
                .filter_module("trace", LevelFilter::Trace);
        }
    }
    // настраиваем формат сообщений в консоли
    builder.format(|buf, record| {
        let (label, color) = match record.target() {
            // задаем цвета
            "critical" => ("CRIT", "\x1b[31m"),     // красный
            "error" => ("ERROR", "\x1b[38;5;208m"), // оранжевый
            "warning" => ("WARN", "\x1b[33m"),      // жёлтый
            "info" => ("INFO", "\x1b[34m"),         // синий
            "debug" => ("DEBUG", "\x1b[35m"),       // фиолетовый
            "trace" => ("TRACE", "\x1b[90m"),       // серый
            _ => ("LOG", "\x1b[0m"), // запасной вариант для сообщений с неизвестным target
        };
        // "\x1b[0m" сбрасывает цвет после названия уровня, чтобы остальной текст терминала не оставался цветным.
        writeln!(buf, "{}{}\x1b[0m {}", color, label, record.args())
    });
    builder.init();
}
