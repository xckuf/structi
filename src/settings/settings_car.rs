use tokio_postgres::{Error, Client};
use std::io::{self, Write};
use crate::car::car::{delete_car, get_car, update_car};
use crate::models::models::Car;
use crate::prelude::prelude_valid::*;

pub async fn settings_car(client: &Client) -> Result<(), Error> {
    let mut input1 = String::new();

    println!("\n\n\n1 - Редактировать машину");
    println!("2 - Удалить машину");
    println!("3 - Выход");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();

    match input1.as_str().trim() {
        "1" => { //Редактировать машину
            let car_id = loop {
                let mut input = String::new();
                println!("Введите id заказа, данные которой хотите обновить:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(car_id1) => break car_id1,
                    Err(_) => println!("\n\nОшибка: id должен быть числом. Пожалуйста, повторите ввод.\n\n"),
                }
            };

            let mut brand1 = String::new();
            let mut model1 = String::new();
            let mut year1: i32 = 0;
            let mut price1: i32 = 0;
            let mut mileage1: i32 = 0;
            let mut is_new1 = false;

            match get_car(&client, car_id).await {
                Ok(Some(car)) => {
                    brand1 = car.brand;
                    model1 = car.model;
                    year1 = car.year;
                    price1 = car.price;
                    mileage1 = car.mileage;
                    is_new1 = car.is_new;
                }
                Ok(None) => {
                    println!("Машина с ID {} не найден.", car_id);
                }
                Err(err) => {
                    eprintln!("Ошибка при получении машины: {:?}", err);
                }
            }

            println!("\nВведите новую марку автомобиля:");
            old_value(&brand1).await;
            let mut brand_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut brand_input).unwrap();
            let brand = if brand_input.trim().is_empty() {
                brand1.clone()
            } else {
                brand_input.trim().to_string()
            };

            println!("\nВведите новую модель автомобиля:");
            old_value(&model1).await;
            let mut model_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut model_input).unwrap();
            let model = if model_input.trim().is_empty() {
                model1.clone()
            } else {
                model_input.trim().to_string()
            };

            println!("\nВведите новый год автомобиля:");
            old_value(year1).await;
            let mut year_str = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut year_str).unwrap();
            let year: i32 = if year_str.trim().is_empty() {
                year1
            } else {
                year_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Ошибка: бюджет должен быть числом. Используем старое значение.");
                        year1
                    })
            };

            println!("\nВведите новую цену автомобиля:");
            old_value(price1).await;
            let mut price_str = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut price_str).unwrap();
            let price: i32 = if price_str.trim().is_empty() {
                price1
            } else {
                price_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Ошибка: цена должна быть числом. Используем старое значение.");
                        price1
                    })
            };

            println!("\nВведите новый пробег автомобиля:");
            old_value(mileage1).await;
            let mut mileage_str = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut mileage_str).unwrap();
            let mileage: i32 = if mileage_str.trim().is_empty() {
                mileage1
            } else {
                mileage_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Ошибка: бюджет должен быть числом. Используем старое значение.");
                        mileage1
                    })
            };

            println!("\nВведите новое состояние автомобиля: \n\
                    0 (Новый автомобиль) или 1 (Не новый автомобиль)");
            old_value(is_new1).await;
            let mut is_new_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut is_new_input).unwrap();
            match is_new_input.trim() {
                "0" => is_new1 = true,
                "1" => is_new1 = false,
                _ => {
                    println!("Неверный ввод. Введите 0 или 1.");
                }
            }
            let is_new = is_new1;

            let updated_car = Car {
                id: Some(car_id),
                brand,
                model,
                year,
                price,
                mileage,
                is_new
            };

            match update_car(&client, car_id, updated_car).await {
                Ok(message) => {
                    println!("{}", message);
                    Ok(())
                },
                Err(err) => {
                    eprintln!("Ошибка при обновлении машины: {:?}", err);
                    Err(err)
                }
            }
        },
        "2" => {//Удалить автомобиль
            let mut input_str = String::new();

            println!("Введите id машины:");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_str).unwrap();

            let input2 = from_str_to_int(input_str.trim());

            match delete_car(&client, input2.await).await {
                Ok(message) => {
                    println!("{}", message);
                    Ok(())
                },
                Err(err) => {
                    eprintln!("Ошибка при удалении автомобиля: {:?}", err);
                    Err(err)
                }
            }
        },
        "3" => Ok(()),
        _ => {
            input_error().await;
            Ok(())
        }
    }
}