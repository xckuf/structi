use std::fmt::Display;

pub async fn from_str_to_int(input: &str) -> i32 {
    input.trim().parse::<i32>().unwrap_or_else(|_| {
        println!("Ошибка: введите корректное число");
        0
    })
}

pub async fn input_error() {
    println!("Неверный ввод")
}

pub async fn old_value<T: Display>(old_value: T) {
    println!("Старое значение: {}", old_value);
}