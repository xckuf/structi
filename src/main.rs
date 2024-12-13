use tokio_postgres::{NoTls, Error};
use std::io::{self, Write};
use dotenv::dotenv;
use std::env;
use chrono::Local;

mod prelude;
use prelude::prelude_main::*;

mod customer;
mod employee;
mod order;
mod car;
mod models;
mod validation;
mod enums;
mod settings;
mod sleep;
mod search;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let mut current_id_empl: i32 = 0;
    let mut current_id_cust: i32 = 0;

    // Employee
    client.execute(
        "CREATE TABLE IF NOT EXISTS employee (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            position VARCHAR(50) NOT NULL,
            salary INTEGER NOT NULL,
            hire_date VARCHAR(20) NOT NULL
        )",
        &[],
    ).await?;

    // Customer
    client.execute(
        "CREATE TABLE IF NOT EXISTS customer (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            phone VARCHAR(50) NOT NULL,
            email VARCHAR(100) ,
            budget INTEGER NOT NULL
        )",
        &[],
    ).await?;

    // Car
    client.execute(
        "CREATE TABLE IF NOT EXISTS car (
            id SERIAL PRIMARY KEY,
            brand VARCHAR(100) NOT NULL,
            model VARCHAR(100) NOT NULL,
            year INTEGER NOT NULL,
            price INTEGER NOT NULL,
            mileage INTEGER NOT NULL,
            is_new BOOLEAN NOT NULL DEFAULT FALSE
        )",
        &[],
    ).await?;

    // Order
    client.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id SERIAL PRIMARY KEY,
            car_id INTEGER NOT NULL,
            customer_id INTEGER NOT NULL,
            employee_id INTEGER NOT NULL,
            order_date VARCHAR(20) NOT NULL,
            price INTEGER NOT NULL,
            is_active BOOLEAN DEFAULT true
        )",
        &[],
    ).await?;

    loop {
        let mut input = String::new();

        println!("1 - Войти\n2 - Создать сотрудника");
        io::stdin().read_line(&mut input).expect("Ошибка при чтении");

        match input.as_str().trim() {
            "1" => { //войти
                println!("Напишите ваш id: ");
                let mut id = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).unwrap();
                id = id.trim().parse().unwrap();
                current_id_empl = id.trim().parse().expect("Ошибка");

                let name = get_employee(&client, current_id_empl).await;

                match name {
                    Ok(Some(employee)) => {
                        println!("\n\n\n\nid: {}", employee.id.unwrap_or_default());
                        println!("name: {}", employee.name);
                        println!("position: {}", employee.position);
                        println!("salary: {}", employee.salary);
                        println!("hire_date: {}", employee.hire_date);
                    },
                    Ok(None) => {
                        println!("Такого сотрудника не существует.");
                        std::process::exit(1);
                    },
                    Err(err) => {
                        println!("Ошибка при выполнении запроса: {:?}", err);
                        std::process::exit(1);
                    }
                }
                break;
            },
            "2" => {//создать работника
                let name1 = loop {
                    let mut input = String::new();
                    println!("\n\n\nВведите Ваше имя:");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    let trimmed = input.trim();
                    if !trimmed.is_empty() {
                        break trimmed.to_string();
                    } else {
                        println!("\n\nОшибка: Ваше имя не может быть пустым. Пожалуйста, повторите ввод.");
                    }
                };

                let position1 = loop {
                    let mut input = String::new();
                    println!("\n\n\nВведите Вашу должность:");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    let trimmed = input.trim();
                    if !trimmed.is_empty() {
                        break trimmed.to_string();
                    } else {
                        println!("\n\nОшибка: Ваша должность не может быть пустой. Пожалуйста, повторите ввод.");
                    }
                };

                let salary1 = loop {
                    let mut input = String::new();
                    println!("\n\n\nВведите Вашу зарплату:");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    match input.trim().parse::<i32>() {
                        Ok(salary) => break salary,
                        Err(_) => println!("\n\nОшибка: зарплата должна быть числом. Пожалуйста, повторите ввод.\n\n"),
                    }
                };

                let date = Local::now().format("%d.%m.%Y").to_string();

                let new_empl = Employee {
                    id: None,
                    name: name1,
                    position: position1,
                    salary: salary1,
                    hire_date: date,
                };

                match create_employee(&client, new_empl).await {
                    Ok(employee_id) => {
                        println!("\n\n\nНовый сотрудник создан с id: {}", employee_id);
                        current_id_empl = employee_id;
                    },
                    Err(err) => eprintln!("\n\n\nОшибка при добавлении сотрудника: {:?}", err),
                }
                break;
            },
            _ => {
                println!("\n\n\nНеверный ввод\n\n\n");
                input.clear();
            }
        }

        sleep_700mil().await;
    }

    sleep_700mil().await;

    loop {
        let mut choose = String::new();

        println!("\n\n\n1 - Создать заказ");
        println!("2 - Закрыть заказ");
        println!("3 - Добавить клиента");
        println!("4 - Выбрать клиента");
        println!("5 - Подобрать автомобиль");
        println!("6 - Добавить автомобиль");
        println!("7 - Поиск");
        println!("8 - Настройки");
        println!("9 - Выход");

        io::stdin().read_line(&mut choose).expect("Ошибка");

        match choose.as_str().trim() {
            "1" => { //создать заказ
                loop {
                    match new_order(current_id_cust, current_id_empl, &client).await {
                        Ok(order_id) => {
                            if order_id != 0 {
                                sleep_700mil().await;
                                break
                            } else {
                                println!("Заказ не был создан.");
                                sleep_700mil().await;
                                break
                            }
                        }
                        Err(err) => {
                            eprintln!("Ошибка при создании заказа: {:?}", err);
                            break
                        }
                    }
                }
            },
            "2" => {//закрыть заказ
                match close_order(&client).await {
                    Ok(_) => sleep_700mil().await,
                    Err(_) => sleep_700mil().await,
                }
            },
            "3" => {//добавить клиента
                loop {
                    match add_customer(&client, &mut current_id_cust).await {
                        Ok(customer_id) => {
                            current_id_cust = customer_id;
                            sleep_700mil().await;
                            break
                        },
                        Err(err) => {
                            eprintln!("Не удалось добавить клиента: {:?}", err);
                            break
                        },
                    }
                }
            },
            "4" => { //выбрать клиента
                loop {
                    match choose_customer(&client, &mut current_id_cust).await {
                        Ok(Some(customer)) => {
                            println!("\n\n\nВыбран клиент:");
                            println!("id: {}", customer.id.unwrap_or(0));
                            println!("Имя: {}", customer.name);
                            println!("Телефон: {}", customer.phone);
                            println!(
                                "Email: {}",
                                customer.email.unwrap_or_else(|| "Не указан".to_string())
                            );
                            println!("Бюджет: {}", customer.budget);
                            break;
                        }
                        Ok(None) => {
                            println!("\n\n\nКлиент не был выбран. Попробуйте снова.");
                        }
                        Err(err) => {
                            eprintln!("\n\n\nОшибка при выборе клиента: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "5" => { //подобрать автомобиль
                loop {
                    match find_car(&client).await {
                        Ok(()) => {
                            println!("\n\n\nПодбор автомобиля завершён.");
                            break;
                        }
                        Err(err) => {
                            eprintln!("\n\n\nОшибка при подборе автомобиля: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "6" => { //добавить автомобиль
                loop {
                    match add_car(&client).await {
                        Ok(_) => {
                            sleep_700mil().await;
                            break;
                        }
                        Err(err) => {
                            eprintln!("Ошибка при добавлении автомобиля: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "7" => { //поиск
                loop {
                    match search(&client).await {
                        Ok(_) => break,
                        Err(err) => {
                            eprintln!("Ошибка при выполнении поиска: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "8" => {//настройки
                loop {
                    match settings(&client, current_id_empl).await {
                        Ok(_) => {
                            println!("Возврат в главное меню...");
                            sleep_700mil().await;
                            break;
                        }
                        Err(err) => {
                            eprintln!("Ошибка в настройках: {:?}", err);
                            sleep_700mil().await;
                            break;
                        }
                    }
                }
            },
            "9" => {
                println!("\n\n\nВсего хорошего! До свидания!");
                break
            }
            _ => input_error().await
        }
    }
    Ok(())
}


