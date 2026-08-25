use chrono::NaiveDateTime;

/// Данные одного измерения.
#[derive(Debug)]
pub struct Measurement {
    pub object_id: i64,
    pub measure_type_id: i64,
    pub value: f64,
    pub timestamp: NaiveDateTime,
}
/// Ошибка при разборе строки CSV.
pub type ParseError = String;
// Разбирает одну строку CSV и создаёт Measurement.
pub fn parse_measurement(line: &str) -> Result<Measurement, ParseError> {
    // Используем csv crate, чтобы правильно обработать
    // CSV и кавычки.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(line.as_bytes());

    let record = match reader.records().next() {
        Some(Ok(record)) => record,
        Some(Err(error)) => {
            return Err(format!("Ошибка парсинга CSV: {}", error));
        }
        None => {
            return Err("Пустая строка CSV".to_string());
        }
    };
    // Должно быть ровно 4 поля.
    if record.len() != 4 {
        return Err(format!(
            "Неверное количество полей: ожидалось 4, получено {}",
            record.len()
        ));
    }
    // object_id должен быть положительным целым числом.
    let object_id: i64 = match record[0].parse() {
        Ok(value) if value > 0 => value,
        _ => {
            return Err(format!("Некорректный object_id: '{}'", &record[0]));
        }
    };
    // measure_type_id должен быть положительным целым числом.
    let measure_type_id: i64 = match record[1].parse() {
        Ok(value) if value > 0 => value,
        _ => {
            return Err(format!("Некорректный measure_type_id: '{}'", &record[1]));
        }
    };
    // value должен быть числом с плавающей точкой.
    let value: f64 = match record[2].parse() {
        Ok(value) => value,
        Err(_) => {
            return Err(format!("Некорректное значение value: '{}'", &record[2]));
        }
    };
    // Timestamp должен иметь формат: YYYY-MM-DD HH:MM:SS
    let timestamp = match NaiveDateTime::parse_from_str(&record[3], "%Y-%m-%d %H:%M:%S") {
        Ok(timestamp) => timestamp,
        Err(_) => {
            return Err(format!(
                "Некорректный timestamp: '{}'. Ожидается формат YYYY-MM-DD HH:MM:SS",
                &record[3]
            ));
        }
    };
    Ok(Measurement {
        object_id,
        measure_type_id,
        value,
        timestamp,
    })
}
