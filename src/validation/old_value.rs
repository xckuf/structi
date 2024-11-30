use std::fmt::Display;

pub async fn old_value<T: Display>(old_value: T) {
    println!("Старое значение: {}", old_value);
}