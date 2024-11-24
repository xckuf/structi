use crate::validation::validation::{from_str_to_int};
use std::io::{self, Write};
use tokio_postgres::{ Error, Client };
use crate::models::models::{Order};
use crate::order::order::{create_order};

pub async fn new_order(current_id_cust: i32, current_id_empl: i32, client: &Client) -> Result<i32, Error> {
        if current_id_cust == 0 {
            println!("Для создания заказ необходимо выбрать клиента");
            return Ok(0);
        }


        let mut car = String::new();
        //let mut customer = String::new();
        let mut date = String::new();
        let mut price = String::new();


        println!("Введите id машины:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut car).unwrap();
        let car = from_str_to_int(&car).await;

        // println!("Введите id заказчика:");
        // io::stdout().flush().unwrap();
        // io::stdin().read_line(&mut customer).unwrap();
        // let customer = from_str_to_int(&customer).await;

        println!("Введите дату:");
        println!("Пример 31.11.2011");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut date).unwrap();
        date = date.trim().parse().unwrap();

        println!("Введите цену:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut price).unwrap();
        let price = from_str_to_int(&price).await;

        let mut new_date = String::new();

        let new_order = Order {
            id: None,
            car_id: car,
            customer_id: current_id_cust,
            employee_id: current_id_empl,
            order_date: new_date,
            price: price,
            is_active: true,
        };

        match create_order(&client, new_order).await {
            Ok(order_id) => {
                println!("Новый заказ создан c id: {}", order_id);
                Ok(order_id)
            },
            Err(err) => {
                eprintln!("Ошибка при создании нового заказа: {}", err);
                Err(err)
            }
        }
}