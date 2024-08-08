use std::ffi::OsStr;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

const FILE_SIZE_BASE: f64 = 1e6;        // (1*10^6) константа для перевода из байт в мегабайты

// Функция получения ввода от пользователя
fn get_input(query: &str) -> std::io::Result<String> {
    print!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

// Функция поиска файлов
fn search_files(search_path: &str, filename: &str, extensions: &Vec<String>,    // путь, имя файла, расширения,
                now: &Instant, results_count: &mut i32) {                       // время поиска, счетчик результата

    let is_no_extensions = extensions.is_empty();                          // есть или нет раширения
    let is_empty_filename = filename.is_empty();                           // есть или нет имя файла

    let files = match std::fs::read_dir(search_path) {                      // читатем папку
        Ok(files) => files,                                                 // получаем список файлов
        Err(_) => return
    };

    // Проходимся по файлам которые прочитали
    for entry in files {
        if let Ok(entry) = entry {
            let path = entry.path();    // получаем путь к файлу
            let file_name = convert_os_str(path.file_stem());   // получаем имя файла
            let file_extension = convert_os_str(path.extension());  // получаем расширение файла

            // если это папка
            if path.is_dir() {
                if is_no_extensions && file_name.contains(filename) {
                    file_found(&path, now, results_count);
                }

                search_files(path.to_str().unwrap_or_default(),
                             filename,
                             extensions,
                             now,
                             results_count
                );
            } else if is_empty_filename && extensions.contains(&file_extension) {
                file_found(&path, now, results_count);
            } else if path.is_file() && file_name.contains(filename) {
                if(!is_no_extensions && extensions.contains(&file_extension)) || is_no_extensions {
                    file_found(&path, now, results_count);
                }
            }
        }
    }
}

// Функция перевода из OsStr в String
fn convert_os_str(os_str: Option<&OsStr>) -> String {
    os_str
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default().to_lowercase()
}

// Функция получения данных для поиска от пользователя
fn get_search_data() -> Option<(String, String, Vec<String>)> { // Возвращает кортеж (1 - папка, 2- имя файла, 3- вектор с расширениями)

    // путь по которому надо искать файлы
    let search_path = match get_input("Enter path to the dir to search for file in: ") {
        Ok(path) => path,
        Err(_) => return None
    };

    // имя файла которое нужно искать
    let search_name = match get_input("Enter filename to search (without extension): ") {
        Ok(name) => name,
        Err(_) => return None
    };

    // расширения которые нужно искать
    let search_extensions = match get_input("Enter file extensions separated by space: ") {
        Ok(extensions_string) => get_extensions(extensions_string),
        Err(_) => return None
    };

    // если нет пути или нет имени файла либо расширения
    if search_path.is_empty() || (search_name.is_empty() && search_extensions.is_empty()){
        println!("Enter something!");
        return None;
    };

    Some((search_path.to_lowercase(), search_name.to_lowercase(), search_extensions))
}

// Функция для разделения нескольких рашсирений из строки в вектор
fn get_extensions(extensions_string: String) -> Vec<String> {
    // делим слова по пробелу, переводим в нижний регистр, собираем в коллекцию и возвращаем вектор
    extensions_string.split_whitespace().map(|ext| ext.to_lowercase()).collect()
}

// Функция если найдено что-то
fn file_found(path: &PathBuf, now: &Instant, results_count: &mut i32) {
    *results_count += 1;
    print_path_info(path, now);
}

// функция вывода пути
fn print_path_info(path: &PathBuf, now: &Instant) {
    print!("{} - Found in {} seconds",
           path.display(),
           now.elapsed().as_secs_f64()
    );

    // Получаем размер данных файла
    match std::fs::metadata(path) {
        Ok(metadata) => {
            print!(" - {} Mb\n", metadata.len() as f64 / FILE_SIZE_BASE); // делим размер на константу чтобы получить из байтов мегабайты
        }
        Err(_) => println!()
    }
}

fn main() {
    loop {
        let (path, filename, extensions) = match get_search_data() {
            None => continue,
            Some(data) => data
        };

        println!();

        // Добавим время которое было затрачено на поиск
        // инициализируем время
        let now = Instant::now();

        // Сколько файлов найдено
        let mut results_count = 0;

        // search files
        search_files(
            path.as_str(),
            filename.as_str(),
            &extensions,
            &now,
            &mut results_count
        );

        println!(
            "\nTotal time: {} seconds\n{} matches\n",
                now.elapsed().as_secs_f64(),        // возвращает количество секунд прощедшее с текущего момента
                results_count);
    }
}
