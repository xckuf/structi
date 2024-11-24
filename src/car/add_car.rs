use tokio_postgres::{Error, Client};
use tokio;
use std::io::{self};
use std::option::Option;

use crate::models::models::{Car};
use crate::validation::validation::{from_str_to_int};
use crate::car::car::{create_car};

pub async fn add_car(client: &Client) -> Result<(), Error> {
    let mut brand1 = String::new();
    println!("Введите марку автомобиля:");
    io::stdin().read_line(&mut brand1).unwrap();
    let mut brand = brand1.trim().to_string();

    let mut model1 = String::new();
    println!("Введите модель автомобиля:");
    io::stdin().read_line(&mut model1).unwrap();
    let mut model = model1.trim().to_string();

    let mut year1 = String::new();
    println!("Введите год производства:");
    io::stdin().read_line(&mut year1).unwrap();
    let year1 = from_str_to_int(&year1);

    let mut price1 = String::new();
    println!("Введите цену автомобиля:");
    io::stdin().read_line(&mut price1).unwrap();
    let price1 = from_str_to_int(&price1);

    let mut mileage1 = String::new();
    println!("Введите пробег автомобиля:");
    io::stdin().read_line(&mut mileage1).unwrap();
    let mileage1 = from_str_to_int(&mileage1);

    let mut is_new_str = String::new();
    let mut is_new1: Option<bool> = None;
    println!("Введите 0 (Новый автомобиль) или 1 (Не новый автомобиль):");
    io::stdin().read_line(&mut is_new_str).unwrap();

    match is_new_str.trim() {
        "0" => is_new1 = Some(true),
        "1" => is_new1 = Some(false),
        _ => {
            println!("Неверный ввод. Введите 0 или 1.");
            is_new1 = None;
        }
    }

    let is_new = is_new1.is_some();

    let new_car = Car {
        id: None,
        brand: brand,
        model: model,
        year: year1.await,
        price: price1.await,
        mileage: mileage1.await,
        is_new: is_new,
    };

    match create_car(&client, new_car).await {
        Ok(car_id) => {
            println!("Создан новый автомобиль с id: {}", car_id);
            Ok(())
        },
        Err(err) => {
            eprintln!("Ошибка при добавлении автомобиля: {}", err);
            Err(err)
        }
    }
}