mod cli; //работа с модулем парсинга аргументов командной строки для консольного приложения
use crate::cli::get_args;

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };
    println!("Каталог: {}", args.input_dir.display());
    println!("Размер batch: {}", args.batch_size);
    println!("Уровень логирования: {:?}", args.log_level); // {:?} - вывод через Debug, так проще посмотреть что лежит в LogLevel
}

