use crate::validation::from_str_to_int::{from_str_to_int};
use std::io::{self, Write};
use tokio_postgres::{ Error, Client };
use crate::models::models::{Order};
use crate::order::order::{create_order};
use chrono::Local;


pub async fn new_order(current_id_cust: i32, current_id_empl: i32, client: &Client) -> Result<i32, Error> {
    if current_id_cust == 0 {
        println!("Для создания заказ необходимо выбрать клиента");
        return Ok(0);
    }

    let mut price = String::new();

    let car_id = loop {
        let mut input = String::new();
        println!("\n\n\nВведите id машины:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(car1) => break car1,
            Err(_) => println!("\n\nОшибка: id машины должен быть числом. Пожалуйста, повторите ввод.\n\n"),
        }
    };

    println!("Введите цену:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut price).unwrap();
    let price = from_str_to_int(&price).await;

    let order_date = Local::now().format("%d.%m.%Y").to_string();

    let new_order = Order {
        id: None,
        car_id,
        customer_id: current_id_cust,
        employee_id: current_id_empl,
        order_date,
        price,
        is_active: true,
    };

    match create_order(&client, new_order).await {
        Ok(order_id) => {
            println!("\n\nНовый заказ создан c id: {}", order_id);
            Ok(order_id)
        },
        Err(err) => {
            eprintln!("\n\nОшибка при создании нового заказа: {}", err);
            Err(err)
        }
    }
}