use tokio_postgres::{Error, Client};
use std::io::{self, Write};
use crate::models::models::{Car};
use crate::car::car::{create_car};

pub async fn add_car(client: &Client) -> Result<(), Error> {
    let brand = loop {
        let mut input = String::new();
        println!("\n\n\nВведите марку автомобиля:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            break trimmed.to_string();
        } else {
            println!("\n\nОшибка: марка автомобиля не может быть пустой. Пожалуйста, повторите ввод.");
        }
    };

    let model = loop {
        let mut input = String::new();
        println!("Введите модель автомобиля:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            break trimmed.to_string();
        } else {
            println!("\n\nОшибка: модель автомобиля не может быть пустой. Пожалуйста, повторите ввод.\n\n");
        }
    };

    let year = loop {
        let mut input = String::new();
        println!("Введите год производства:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(year1) => break year1,
            Err(_) => println!("\n\nОшибка: год производства должен быть числом. Пожалуйста, повторите ввод.\n\n"),
        }
    };

    let price = loop {
        let mut input = String::new();
        println!("Введите цену автомобиля:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(price1) => break price1,
            Err(_) => println!("\n\nОшибка: цена автомобиля должна быть числом. Пожалуйста, повторите ввод.\n\n"),
        }
    };

    let mileage = loop {
        let mut input = String::new();
        println!("Введите пробег автомобиля:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(mileage1) => break mileage1,
            Err(_) => println!("\n\nОшибка: пробег автомобиля должен быть числом. Пожалуйста, повторите ввод.\n\n"),
        }
    };

    let is_new1 = loop {
        let mut is_new_str = String::new();
        println!("Введите 0 (Новый автомобиль) или 1 (Не новый автомобиль):");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut is_new_str).unwrap();

        match is_new_str.trim() {
            "0" => break Some(true),
            "1" => break Some(false),
            _ => {
                println!("\n\nОшибка: неверный ввод. Введите 0 или 1.\n\n");
            }
        }
    };

    let is_new = is_new1.is_some();

    let new_car = Car {
        id: None,
        brand,
        model,
        year,
        price,
        mileage,
        is_new,
    };

    match create_car(&client, new_car).await {
        Ok(car_id) => {
            println!("\n\n\nСоздан новый автомобиль с id: {}", car_id);
            Ok(())
        },
        Err(err) => {
            eprintln!("\n\n\nОшибка при добавлении автомобиля: {}", err);
            Err(err)
        }
    }
}