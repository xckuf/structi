use tokio_postgres::{Error, Client};
use std::io::{self};

use crate::customer::customer::{get_customer};
use crate::employee::employee::{get_employee};
use crate::order::order::{get_order};
use crate::car::car::{get_car};
use crate::prelude::prelude_valid::*;
use crate::sleep::sleep_700mil::sleep_700mil;

pub async fn search(client: &Client) -> Result<(), Error> {
    let mut input = String::new();

    println!("\n\n\n1 - Поиск покупателя по id");
    println!("2 - Поиск работника по id");
    println!("3 - Поиск автомобиля по id");
    println!("4 - Поиск заказа по id");
    println!("5 - Выход");

    io::stdin().read_line(&mut input).expect("Ошибка");

    match input.as_str().trim() {
        "1" => {//Поиск покупателя по id
            let mut id = String::new();
            println!("Введите id покупателя:");
            io::stdin().read_line(&mut id).unwrap();
            let id = from_str_to_int(&id);

            match get_customer(&client, id.await).await {
                Ok(Some(customer)) => {
                    match customer.id {
                        Some(id) => println!("\n\n\nid: {}", id),
                        None => println!("\n\n\nid: Не найдено")
                    }
                    println!("Имя: {}", customer.name);
                    println!("Телефон: {}", customer.phone);
                    match customer.email {
                        Some(email) => println!("Email: {}", email),
                        None => println!("Email: Не указано")
                    }
                    println!("Бюджет: {}", customer.budget);
                    sleep_700mil().await;
                    Ok(())
                }
                Ok(None) => {
                    println!("Клиент не найден");
                    sleep_700mil().await;
                    Ok(())
                }
                Err(err) => {
                    eprintln!("Ошибка при получении клиента: {}", err);
                    sleep_700mil().await;
                    Err(err)
                }
            }
        },
        "2" => {//Поиск работника по id
            let mut id = String::new();
            println!("Введите id работника:");
            io::stdin().read_line(&mut id).unwrap();
            let id = from_str_to_int(&id);

            match get_employee(&client, id.await).await {
                Ok(Some(employee)) => {
                    match employee.id {
                        Some(id) => println!("\n\n\nid: {}", id),
                        None => println!("\n\n\nid: Не найдено")
                    }
                    println!("Имя: {}", employee.name);
                    println!("Должность: {}", employee.position);
                    println!("Зарплата: {}", employee.salary);
                    println!("Работает с {}", employee.hire_date);
                    sleep_700mil().await;
                    Ok(())
                }
                Ok(None) => {
                    println!("Работник не найден");
                    sleep_700mil().await;
                    Ok(())
                }
                Err(err) => {
                    eprintln!("Ошибка при получении работника: {}", err);
                    sleep_700mil().await;
                    Err(err)
                }
            }
        },
        "3" => {//Поиск автомобиля по id
            let mut id = String::new();
            println!("Введите id автомобиля:");
            io::stdin().read_line(&mut id).unwrap();
            let id = from_str_to_int(&id);

            match get_car(&client, id.await).await {
                Ok(Some(car)) => {
                    match car.id {
                        Some(id) => println!("\n\n\nid: {}", id),
                        None => println!("\n\n\nid: Не найдено")
                    }
                    println!("Марка: {}", car.brand);
                    println!("Модель: {}", car.model);
                    println!("Год: {}", car.year);
                    println!("Цена: {}", car.price);
                    println!("Пробег: {}", car.mileage);
                    println!("Новый: {}", car.is_new);
                    sleep_700mil().await;
                    Ok(())
                }
                Ok(None) => {
                    println!("Автомобиль не найдена");
                    sleep_700mil().await;
                    Ok(())
                }
                Err(err) => {
                    eprintln!("Ошибка при получении автомобиля: {}", err);
                    sleep_700mil().await;
                    Err(err)
                }
            }
        },
        "4" => {
            let mut id = String::new();
            println!("Введите id автомобиля:");
            io::stdin().read_line(&mut id).unwrap();
            let id = from_str_to_int(&id);

            match get_order(&client, id.await).await {
                Ok(Some(order)) => {
                    match order.id {
                        Some(id) => println!("\n\n\nid: {}", id),
                        None => println!("\n\n\nid: Не найдено")
                    }
                    match order.id {
                        Some(id) => println!("\n\n\nid: {}", id),
                        None => println!("\n\n\nid: Не найдено")
                    }
                    println!("id Клинта: {}", order.customer_id);
                    println!("id Работника: {}", order.employee_id);
                    println!("Дата заказа: {}", order.order_date);
                    println!("Цена: {}", order.price);
                    println!("Активен: {}", order.is_active);
                    sleep_700mil().await;
                    Ok(())
                }
                Ok(None) => {
                    println!("Заказ не найден");
                    sleep_700mil().await;
                    Ok(())
                },
                Err(err) => {
                    eprintln!("Ошибка при получении заказа: {}", err);
                    sleep_700mil().await;
                    Err(err)
                }
            }
        },
        "5" => {
            println!("Выход из поиска.");
            sleep_700mil().await;
            Ok(())
        }
        _ => {
            input_error().await;
            sleep_700mil().await;
            Ok(())
        }
    }
}