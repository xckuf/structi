use tokio_postgres::{Error, Client};
use std::io::{self, Write};
use crate::models::models::Order;
use crate::order::order::{get_order, update_order, delete_order};
use crate::prelude::prelude_valid::*;
pub async fn settings_order(client: &Client) -> Result<(), Error> {
    let mut input1 = String::new();

    println!("\n\n\n1 - Редактировать заказ");
    println!("2 - Удалить заказ");
    println!("3 - Выход");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();

    match input1.as_str().trim() {
        "1" => { //Редактировать заказ
            let order_id = loop {
                let mut input = String::new();
                println!("Введите id заказа, данные которой хотите обновить:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(order_id1) => break order_id1,
                    Err(_) => println!("\n\nОшибка: id должен быть числом. Пожалуйста, повторите ввод.\n\n"),
                }
            };

            let mut car_id1: i32 = 0;
            let mut customer_id1: i32 = 0;
            let mut employee_id1: i32 = 0;
            let mut order_date1 = String::new();
            let mut price1: i32 = 0;
            let mut is_active1 = false;

            match get_order(&client, order_id).await {
                Ok(Some(order)) => {
                    car_id1 = order.car_id;
                    customer_id1 = order.customer_id;
                    employee_id1 = order.employee_id;
                    order_date1 = order.order_date;
                    price1 = order.price;
                    is_active1 = order.is_active;
                }
                Ok(None) => {
                    println!("Заказ с id {} не найден.", order_id);
                }
                Err(err) => {
                    eprintln!("Ошибка при получении заказа: {:?}", err);
                }
            }

            println!("\nВведите car_id:");
            old_value(car_id1).await;
            let mut car_id_str = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut car_id_str).unwrap();
            let car_id: i32 = if car_id_str.trim().is_empty() {
                car_id1
            } else {
                car_id_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Ошибка: car_id должен быть числом. Используем старое значение.");
                        car_id1
                    })
            };

            println!("\nВведите customer_id:");
            old_value(customer_id1).await;
            let mut customer_id_str = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut customer_id_str).unwrap();
            let customer_id: i32 = if customer_id_str.trim().is_empty() {
                customer_id1
            } else {
                customer_id_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Ошибка: customer_id должен быть числом. Используем старое значение.");
                        customer_id1
                    })
            };

            println!("\nВведите employee_id:");
            old_value(employee_id1).await;
            let mut employee_id_str = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut employee_id_str).unwrap();
            let employee_id: i32 = if employee_id_str.trim().is_empty() {
                employee_id1
            } else {
                employee_id_str
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Ошибка: customer_id должен быть числом. Используем старое значение.");
                        employee_id1
                    })
            };

            println!("\nВведите новую дату, когда создали заказ:");
            old_value(&order_date1).await;
            let mut order_date = String::new();
            loop {
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut order_date).unwrap();
                order_date = order_date.trim().to_string();

                if order_date.is_empty() {
                    order_date = order_date1.clone();
                    break;
                }

                match valid_date(order_date.clone()).await {
                    Ok(_) => break,
                    Err(err) => {
                        println!("Ошибка: {}", err);
                        println!("Попробуйте снова. Введите дату в формате дд.мм.гггг:");
                        order_date.clear();
                    }
                }
            }

            println!("\nВведите новую сумму заказа:");
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
                        eprintln!("Ошибка: сумма должна быть числом. Используем старое значение.");
                        price1
                    })
            };

            println!("\nВведите новое состояние заказа: \n\
                    0 (Актуальный) или 1 (Неактуальный)");
            old_value(is_active1).await;
            let mut is_active_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut is_active_input).unwrap();
            match is_active_input.trim() {
                "0" => is_active1 = true,
                "1" => is_active1 = false,
                _ => {
                    println!("Неверный ввод. Введите 0 или 1.");
                }
            }
            let is_active = is_active1;

            let updated_order = Order {
                id: Some(order_id),
                car_id,
                customer_id,
                employee_id,
                order_date,
                price,
                is_active
            };

            match update_order(&client, order_id, updated_order).await {
                Ok(message) => {
                    println!("{}", message);
                    Ok(())
                },
                Err(err) => {
                    eprintln!("Ошибка при обновлении заказа: {:?}", err);
                    Err(err)
                }
            }
        },
        "2" => {//Удалить заказ
            let mut input_str = String::new();

            println!("Введите id заказа:");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_str).unwrap();

            let input2 = from_str_to_int(input_str.trim());

            match delete_order(&client, input2.await).await {
                Ok(message) => {
                    println!("{}", message);
                    Ok(())
                },
                Err(err) => {
                    eprintln!("Ошибка при удалении заказа: {:?}", err);
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