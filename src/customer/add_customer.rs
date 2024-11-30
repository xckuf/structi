use std::io::{self, Write};
use tokio_postgres::{ Error, Client };
use crate::models::models::{Customer};
use crate::customer::customer::{create_customer};
pub use crate::validation::valid_email::valid_email;

pub async fn add_customer(client: &Client, current_id_cust: &mut i32) -> Result<i32, Error> {
    let name1 = loop {
        let mut input = String::new();
        println!("\n\n\nВведите имя клиента:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            break trimmed.to_string();
        } else {
            println!("\n\nОшибка: имя клиента не может быть пустым. Пожалуйста, повторите ввод.");
        }
    };

    let phone1 = loop {
        let mut input = String::new();
        println!("Введите телефон клиента:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            break trimmed.to_string();
        } else {
            println!("\n\nОшибка: телефон клиента не может быть пустым. Пожалуйста, повторите ввод.\n\n");
        }
    };

    let email1 = loop {
        let mut input = String::new();
        println!("Введите email клиента(необязательно):");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if trimmed.is_empty() {
            break None;
        } else {
            match valid_email(trimmed.to_string()).await {
                Ok(_) => break Some(trimmed.to_string()),
                Err(err) => println!("\n\nОшибка: {}. Пожалуйста, повторите ввод.\n\n", err),
            }
        }
    };

    let budget1 = loop {
        let mut input = String::new();
        println!("Введите бюджет клиента:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(budget) => break budget,
            Err(_) => println!("\n\nОшибка: бюджет должен быть числом. Пожалуйста, повторите ввод.\n\n"),
        }
    };

    let new_cust = Customer {
        id: None,
        name: name1,
        phone: phone1,
        email: email1,
        budget: budget1,
    };

    match create_customer(&client, new_cust).await {
        Ok(customer_id) => {
            println!("\n\n\nСоздан новый клиент с id: {}", customer_id);
            *current_id_cust = customer_id;
            Ok(customer_id)
        },
        Err(err) => {
            println!("Ошибка при создании нового клиента: {}", err);
            Err(err)
        }
    }
}