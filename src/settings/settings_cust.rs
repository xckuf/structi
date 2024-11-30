use tokio_postgres::{Error, Client};
use std::io::{self, Write};
use std::option::Option;
use crate::customer::add_customer::valid_email;
use crate::customer::customer::{update_customer, get_customer, delete_customer};
use crate::models::models::Customer;
use crate::prelude::prelude_valid::*;


pub async fn settings_cust(client: &Client) -> Result<(), Error> {
    let mut input1 = String::new();

    println!("\n\n\n1 - Редактировать покупателя");
    println!("2 - Удалить покупателя");
    println!("3 - Выход");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();

    match input1.as_str().trim() {
        "1" => { //редактировать покупателя
            let customer_id = loop {
                let mut input = String::new();
                println!("Введите id клиента, данные которой хотите обновить:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(customer_id1) => break customer_id1,
                    Err(_) => println!("\n\nОшибка: id должен быть числом. Пожалуйста, повторите ввод.\n\n"),
                }
            };

            let mut name1 = String::new();
            let mut phone1 = String::new();
            let mut email1: Option<String> = Option::from(String::new());
            let mut budget1: i32 = 0;

            match get_customer(&client, customer_id).await {
                Ok(Some(customer)) => {
                    name1 = customer.name;
                    phone1 = customer.phone;
                    email1 = customer.email;
                    budget1 = customer.budget;
                }
                Ok(None) => {
                    println!("Клиент с ID {} не найден.", customer_id);
                }
                Err(err) => {
                    eprintln!("Ошибка при получении клиента: {:?}", err);
                }
            }
            println!("\nВведите новое имя клиента:");
            old_value(&name1).await;
            let mut name_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut name_input).unwrap();
            let name = if name_input.trim().is_empty() {
                name1.clone()
            } else {
                name_input.trim().to_string()
            };


            println!("\nВведите новый номер телефона клиента:");
            old_value(&phone1).await;
            let mut phone_input  = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut phone_input ).unwrap();
            let phone = if phone_input.trim().is_empty() {
                phone1.clone()
            } else {
                phone_input.trim().to_string()
            };

            println!("\nВведите новый email клиента:");
            old_value(email1.clone().unwrap_or_else(|| "Не указан".to_string())).await;
            let mut email_input  = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut email_input ).unwrap();
            let email = if email_input.trim().is_empty() {
                email1.clone()
            } else {
                let email_input_trimmed = email_input.trim().to_string();

                match valid_email(email_input_trimmed.clone()).await {
                    Ok(_) => Some(email_input_trimmed),
                    Err(err) => {
                        println!("Ошибка: {}", err);
                        None
                    }
                }
            };

            println!("\nВведите новый бюджет клиента:");
            old_value(budget1).await;
            let mut budget_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut budget_input).unwrap();
            let budget: i32 = if budget_input.trim().is_empty() {
                budget1
            } else {
                budget_input
                    .trim()
                    .parse()
                    .unwrap_or_else(|_| {
                        eprintln!("Ошибка: бюджет должен быть числом. Используем старое значение.");
                        budget1
                    })
            };

            let updated_customer = Customer {
                id: Some(customer_id),
                name,
                phone,
                email,
                budget,
            };

            match update_customer(&client, customer_id, updated_customer).await {
                Ok(message) => {
                    println!("{}", message);
                    Ok(())
                },
                Err(err) => {
                    eprintln!("Ошибка при обновлении клиента: {:?}", err);
                    Err(err)
                },
            }
        },
        "2" => { //удалить покупателя
            let mut input_str = String::new();

            println!("Введите id покупателя:");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input_str).unwrap();

            let input2 = from_str_to_int(input_str.trim()).await;

            match delete_customer(&client, input2).await {
                Ok(message) => {
                    println!("{}", message);
                    Ok(())
                },
                Err(err) => {
                    eprintln!("Ошибка при удалении клиента: {:?}", err);
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