use chrono::NaiveDateTime;
use log::error;

/// Данные одного измерения.
#[derive(Debug)]
pub struct Measurement {
    pub object_id: i64,
    pub measure_type_id: i64,
    pub value: f64,
    pub timestamp: NaiveDateTime,
}
// Разбирает одну строку CSV и создаёт Measurement.
// Если строка неправильная, возвращается None и записывается сообщение об ошибке.
pub fn parse_measurement(line: &str) -> Option<Measurement> {
    // Используем csv crate, чтобы правильно обработать
    // CSV и кавычки.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());

    let record = match reader.records().next() {
        Some(Ok(record)) => record,
        Some(Err(error)) => {
            error!(
                target: "error",
                "Ошибка парсинга CSV: {}",
                error
            );
            return None;
        }
        None => {
            error!(
                target: "error",
                "Пустая строка CSV"
            );
            return None;
        }
    };
    // Должно быть ровно 4 поля.
    if record.len() != 4 {
        error!(
            target: "error",
            "Неверное количество полей: ожидалось 4, получено {}",
            record.len()
        );
        return None;
    }
    // object_id должен быть положительным целым числом.
    let object_id: i64 = match record[0].parse() {
        Ok(value) if value > 0 => value,
        _ => {
            error!(
                target: "error",
                "Некорректный object_id: '{}'",
                &record[0]
            );
            return None;
        }
    };
    // measure_type_id должен быть положительным целым числом.
    let measure_type_id: i64 = match record[1].parse() {
        Ok(value) if value > 0 => value,
        _ => {
            error!(
                target: "error",
                "Некорректный measure_type_id: '{}'",
                &record[1]
            );
            return None;
        }
    };
    // value должен быть числом с плавающей точкой.
    let value: f64 = match record[2].parse() {
        Ok(value) => value,
        Err(_) => {
            error!(
                target: "error",
                "Некорректное значение value: '{}'",
                &record[2]
            );
            return None;
        }
    };
    // Timestamp должен иметь формат: YYYY-MM-DD HH:MM:SS
    let timestamp = match NaiveDateTime::parse_from_str(&record[3], "%Y-%m-%d %H:%M:%S") {
        Ok(timestamp) => timestamp,
        Err(_) => {
            error!(
                target: "error",
                "Некорректный timestamp: '{}'. Ожидается формат YYYY-MM-DD HH:MM:SS",
                &record[3]
            );
            return None;
        }
    };
    Some(Measurement {
        object_id,
        measure_type_id,
        value,
        timestamp,
    })
}
