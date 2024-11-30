pub async fn from_str_to_int(input: &str) -> i32 {
    input.trim().parse::<i32>().unwrap_or_else(|_| {
        println!("Ошибка: введите корректное число");
        0
    })
}

