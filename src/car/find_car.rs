use tokio_postgres::{Error, Client};
use tokio;
use std::io::{self, Write};
use std::option::Option;
use crate::car::car::{budget_car_do, budget_car_ot, search_cars};
use crate::models::models::Car;

pub async fn find_car(client: &Client, ) -> Result<(), Error> {
    let mut int = String::new();
    println!("1 - По бюджету");
    println!("2 - По определенным фильтрам");
    println!("3 - Выход");

    io::stdin().read_line(&mut int).expect("Ошибка");



    match int.as_str().trim() {
        "1" => { //по бюджету
            let mut int1 = String::new();

            println!("1 - До");
            println!("2 - От");

            io::stdin().read_line(&mut int1).expect("Ошибка");

            match int1.as_str().trim() {
                "1" => { //до
                    let mut budget1 = String::new();
                    println!("Введите сумму до которой Вы готовы приобрести автомобиль:");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut budget1).expect("Ошибка при чтении ввода");

                    match budget1.trim().parse::<i32>() {
                        Ok(budget) => {
                            match budget_car_do(&client, budget).await {
                                Ok(cars) => {
                                    if cars.is_empty() {
                                        println!("Нет автомобилей с ценой до {}", budget);
                                    } else {
                                        println!("Автомобили в бюджете до {}:", budget);
                                        for car in cars {
                                            println!("id: {}", car.id.unwrap_or(0));
                                            println!("Марка: {}", car.brand);
                                            println!("Модель: {}", car.model);
                                            println!("Год: {}", car.year);
                                            println!("Цена: {}", car.price);
                                            println!("Пробег: {}", car.mileage);
                                            println!("Новый: {}", if car.is_new { "Да" } else { "Нет" });
                                            println!();
                                        }
                                    }
                                }
                                Err(err) => eprintln!("Ошибка при получении автомобилей: {:?}", err),
                            }
                        }
                        Err(_) => {
                            println!("Пожалуйста, введите корректное число.");
                        }
                    }
                },
                "2" => { //от
                    let mut budget1 = String::new();
                    println!("Введите сумму от которой Вы готовы приобрести автомобиль:");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut budget1).expect("Ошибка при чтении ввода");

                    match budget1.trim().parse::<i32>() {
                        Ok(budget) => {
                            match budget_car_ot(&client, budget).await {
                                Ok(cars) => {
                                    if cars.is_empty() {
                                        println!("Нет автомобилей с ценой от {}", budget);
                                    } else {
                                        println!("Автомобили в бюджете от {}:", budget);
                                        for car in cars {
                                            println!("id: {}", car.id.unwrap_or(0));
                                            println!("Марка: {}", car.brand);
                                            println!("Модель: {}", car.model);
                                            println!("Год: {}", car.year);
                                            println!("Цена: {}", car.price);
                                            println!("Пробег: {}", car.mileage);
                                            println!("Новый: {}", if car.is_new { "Да" } else { "Нет" });
                                            println!();
                                        }
                                    }
                                }
                                Err(err) => eprintln!("Ошибка при получении автомобилей: {:?}", err),
                            }
                        }
                        Err(_) => {
                            println!("Пожалуйста, введите корректное число.");
                        }
                    }
                },
                _ => println!("Введите корректное значение")
            }
        },
        "2" => { //определенные фильтры для автомобиля
            let mut input = String::new();

            let mut id1: Option<i32> = None;
            let mut brand1: Option<String> = None;
            let mut model1: Option<String> = None;
            let mut year1: Option<i32> = None;
            let mut price1: Option<i32> = None;
            let mut mileage1: Option<i32> = None;
            let mut is_new: Option<bool> = None;

            loop {
                println!("1) id: {:?}", id1);
                println!("2) Марка: {:?}", brand1);
                println!("3) Модель: {:?}", model1);
                println!("4) Год: {:?}", year1);
                println!("5) Цена: {:?}", price1);
                println!("6) Пробег: {:?}", mileage1);
                println!("7) Новый: {:?}", is_new);
                println!("8) Готово");

                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                match input.as_str().trim() {
                    "1" => {
                        let mut id_str = String::new();
                        println!("Введите id:");
                        io::stdin().read_line(&mut id_str).unwrap();
                        id1 = id_str.trim().parse::<i32>().ok();
                    },
                    "2" => {
                        let mut brand_str = String::new();
                        println!("Введите марку:");
                        io::stdin().read_line(&mut brand_str).unwrap();
                        brand1 = Some(brand_str.trim().to_string());
                    },
                    "3" => {
                        let mut model_str = String::new();
                        println!("Введите телефон:");
                        io::stdin().read_line(&mut model_str).unwrap();
                        model1 = Some(model_str.trim().to_string());
                    },
                    "4" => {
                        let mut year_str = String::new();
                        println!("Введите год:");
                        io::stdin().read_line(&mut year_str).unwrap();
                        year1 = year_str.trim().parse::<i32>().ok();
                    },
                    "5" => {
                        let mut price_str = String::new();
                        println!("Введите бюджет:");
                        io::stdin().read_line(&mut price_str).unwrap();
                        price1 = price_str.trim().parse::<i32>().ok();
                    },
                    "6" => {
                        let mut mileage_str = String::new();
                        println!("Введите пробег");
                        io::stdin().read_line(&mut mileage_str).unwrap();
                        mileage1 = mileage_str.trim().parse::<i32>().ok();
                    },
                    "7" => {
                        let mut is_new_str = String::new();
                        println!("Введите 0 (Новый автомобиль) или 1 (Не новый автомобиль):");
                        io::stdin().read_line(&mut is_new_str).unwrap();

                        match is_new_str.trim() {
                            "0" => is_new = Some(true),
                            "1" => is_new = Some(false),
                            _ => {
                                println!("Неверный ввод. Введите 0 или 1.");
                                is_new = None;
                            }
                        }
                    },
                    "8" => break,
                    _ => println!("Неверный ввод")
                }
                match search_cars(&client, id1, brand1.clone(), model1.clone(), year1, price1, mileage1, is_new).await {
                    Ok(cars) => {
                        if cars.is_empty() {
                            println!("Автомобили не найдены")
                        } else {
                            for car in cars {
                                println!("{}", car.pretty_print_car());
                            }
                        }
                    }
                    Err(err) => eprintln!("Ошибка при поиске автомобиля: {:?}", err)
                }
            }
        },
        "3" => {
            println!("Выход из поиска автомобилей.");
            return Ok(());
        }
        _ => println!("Некорректный выбор. Попробуйте снова."),
    }
    Ok(())
}