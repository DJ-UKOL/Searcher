use std::io::Write;

// Функция получения ввода от пользователя
fn get_input(query: &str) -> std::io::Result<String> {
    print!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

// Функция получения данных для поиска от пользователя
fn get_search_data() -> Option<(String, String, Vec<String>)> { // 1 - папка, 2- имя файла, 3- расширение
    // путь по которому надо искать файлы
    let search_path = match get_input("Enter path to the dir to search for file in: ") {
        Ok(path) => path,
        Err(_) => return None
    };
    // имя файла которое нужно искать
    let search_name = match get_input("Enter filename to search (without extension: ") {
        Ok(name) => name,
        Err(_) => return None
    };

    // расширение которое нужно искать
    let search_extensions = match get_input("Enter file extensions separated by space: ") {
        Ok(extensions_string) => get_extensions(extensions_string),
        Err(_) => return None
    };

    if search_path.is_empty() || (search_name.is_empty() && search_extensions.is_empty()){
        println!("Enter something!");
        return None;
    }

    Some((search_path.to_lowercase(), search_name.to_lowercase(), search_extensions))
}

// Функция для разделения нескольких рашсирений из строки в вектор
fn get_extensions(extensions_string: String) -> Vec<String> {
    // делим слова по пробелу, переводим в нижний регистр, и собираем в коллекцию
    extensions_string.split_whitespace().map(|ext| ext.to_lowercase()).collect()
}



fn main() {
    println!("Hello, world!");
}
