use std::io::{self, Write};
use tokio_postgres::{ Error, Client };
use crate::validation::validation::{from_str_to_int};
use crate::models::models::{Customer};
use crate::customer::customer::{create_customer};

pub async fn add_customer(client: &Client, current_id_cust: &mut i32) -> Result<i32, Error> {
    let mut name1 = String::new();
    let mut phone1 = String::new();
    let mut email1 = String::new();
    let mut budget1 = String::new();

    println!("Введите имя клиента:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name1).unwrap();
    let name1 = name1.trim().parse().unwrap();

    println!("Введите телефон клиента:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut phone1).unwrap();
    let phone1 = phone1.trim().parse().unwrap();

    println!("Введите email клиента(необязательно):");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut email1).unwrap();
    let email1 = match email1.trim() {
        "" => None,
        value => Some(value.to_string()),
    };

    println!("Введите бюджет клиента:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut budget1).unwrap();
    let budget1 = from_str_to_int(&budget1).await;

    let new_cust = Customer {
        id: None,
        name: name1,
        phone: phone1,
        email: email1,
        budget: budget1,
    };

    match create_customer(&client, new_cust).await {
        Ok(customer_id) => {
            println!("Создан новый клиент с id: {}", customer_id);
            *current_id_cust = customer_id;
            Ok(customer_id)
        },
        Err(err) => {
            println!("Ошибка при создании нового клиента: {}", err);
            Err(err)
        }
    }
}