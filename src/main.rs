use tokio_postgres::{NoTls, Error, GenericClient};
use tokio;
use std::io::{self, Write};
use std::{time::Duration};
use async_std::task;
use dotenv::dotenv;
use std::env;

use crate::order::new_order::new_order;
use crate::customer::add_customer::add_customer;
use crate::customer::choose_customer::choose_customer;
use crate::car::find_car::find_car;
use crate::car::add_car::add_car;
use crate::validation::search::search;
use crate::validation::settings::settings;
use crate::employee::employee::{create_employee, get_employee};
use crate::models::models::{Employee};
use crate::validation::validation::{input_error};

mod customer;
mod employee;
mod order;
mod car;
mod models;
mod validation;

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

        println!("1 - Войти\n2 - Создать работника");
        io::stdin().read_line(&mut input).expect("Ошибка при чтении");

        match input.as_str().trim() {
            "1" => { // sign in
                println!("Напишите ваш id: ");
                let mut id = String::new();
                io::stdout().flush().unwrap();  //чтобы вывелся println
                io::stdin().read_line(&mut id).unwrap(); //ожидает ввод
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
                    Ok(None) => println!("Такого сотрудника не существует."),
                    Err(err) => println!("Ошибка при выполнении запроса: {:?}", err)
                }
                break;
            },
            "2" => {
                println!("Напишите Ваше имя:");
                let mut name1 = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut name1).unwrap();
                name1 = name1.trim().parse().unwrap();

                println!("Напишите Вашу должность:");
                let mut position1 = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut position1).unwrap();
                position1 = position1.trim().parse().unwrap();

                println!("Напишите Вашу зарплату:");
                let mut salary1_str = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut salary1_str).unwrap();
                let salary1: i32 = salary1_str.trim().parse().expect("Ошибка с числом ..130");

                let date = String::new();

                let new_empl = Employee {
                    id: None,
                    name: name1,
                    position: position1,
                    salary: salary1,
                    hire_date: date,
                };

                match create_employee(&client, new_empl).await {
                    Ok(employee_id) => {
                        println!("Новый сотрудник создан с ID: {}", employee_id);
                        current_id_empl = employee_id;
                    },
                    Err(err) => eprintln!("Ошибка при добавлении сотрудника: {:?}", err),
                }
                break;
            },
            _ => {
                println!("Неверный ввод");
                input.clear();
            }
        }

        task::sleep(Duration::from_secs(1)).await;
    }

    task::sleep(Duration::from_secs(1)).await;

    loop {
        let mut choose = String::new();

        println!("\n\n\n1 - Создать заказ");
        println!("2 - Добавить клиента");
        println!("3 - Выбрать клиента");
        println!("4 - Подобрать автомобиль");
        println!("5 - Добавить автомобиль");
        println!("6 - Поиск");
        println!("7 - Настройки");
        println!("8 - Выход");

        io::stdin().read_line(&mut choose).expect("Ошибка");

        match choose.as_str().trim() {
            "1" => { //создать заказ
                loop {
                    match new_order(current_id_cust, current_id_empl, &client).await {
                        Ok(order_id) => {
                            if order_id != 0 {
                                println!("Создан новый заказ с ID: {}", order_id);
                                break
                            } else {
                                println!("Заказ не был создан.");
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
            "2" => {//добавить клиента
                loop {
                    match add_customer(&client, &mut current_id_cust).await {
                        Ok(customer_id) => {
                            println!("Новый клиент с ID {} успешно добавлен!", customer_id);
                            break
                        },
                        Err(err) => {
                            eprintln!("Не удалось добавить клиента: {:?}", err);
                            break
                        },
                    }
                }
            },
            "3" => { //выбрать клиента
                loop {
                    match choose_customer(&client, &mut current_id_cust).await {
                        Ok(Some(customer)) => {
                            println!("Выбран клиент:");
                            println!("ID: {}", customer.id.unwrap_or(0));
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
                            println!("Клиент не был выбран. Попробуйте снова.");
                        }
                        Err(err) => {
                            eprintln!("Ошибка при выборе клиента: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "4" => { //подобрать автомобиль
                loop {
                    match find_car(&client).await {
                        Ok(()) => {
                            println!("Подбор автомобиля завершён.");
                            break;
                        }
                        Err(err) => {
                            eprintln!("Ошибка при подборе автомобиля: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "5" => { //добавить автомобиль
                loop {
                    match add_car(&client).await {
                        Ok(_) => {
                            println!("Добавление автомобиля завершено.");
                            break;
                        }
                        Err(err) => {
                            eprintln!("Ошибка при добавлении автомобиля: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "6" => {
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
            "7" => {//настройки
                loop {
                    match settings(&client).await {
                        Ok(_) => {
                            println!("Возврат в главное меню...");
                        }
                        Err(err) => {
                            eprintln!("Ошибка в настройках: {:?}", err);
                            break;
                        }
                    }
                }
            },
            "8" => {
                println!("\n\n\nВсего хорошего! До свидания!");
                break
            }
            _ => input_error().await
        }
    }
    Ok(())
}


