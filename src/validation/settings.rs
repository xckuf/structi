use tokio_postgres::{Error, Client};
use tokio;
use std::io::{self, Write};
use std::option::Option;
use crate::customer::customer::{update_customer, get_customer, delete_customer};

use crate::models::models::{Customer};

use crate::validation::validation::{from_str_to_int, input_error, old_value};

pub async fn settings(client: &Client) -> Result<(), Error> {
    let mut input = String::new();

    println!("1 - Клиент");
    println!("2 - Работник");
    println!("3 - Автомобиль");
    println!("4 - Заказ");
    println!("5 - Выход");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    match input.as_str().trim() {
        "1" => { //клиент
            let mut input1 = String::new();

            println!("1 - Редактировать покупателя");
            println!("2 - Удалить покупателя");
            println!("3 - Выход");

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input1).unwrap();

            match input1.as_str().trim() {
                "1" => { //редактировать покупателя
                    println!("Введите ID клиента, данные которого хотите обновить:");
                    let mut customer_id_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut customer_id_input).unwrap();
                    let customer_id: i32 = customer_id_input.trim().parse().expect("Ошибка: ID должен быть числом");

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
                    old_value(name1).await;
                    let mut name = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut name).unwrap();
                    let name = name.trim().to_string();

                    println!("\nВведите новый номер телефона клиента:");
                    old_value(phone1).await;
                    let mut phone = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut phone).unwrap();
                    let phone = phone.trim().to_string();

                    println!("\nВведите новый email клиента:");
                    old_value(email1.clone().unwrap_or_else(|| "Не указан".to_string())).await;
                    let mut email = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut email).unwrap();
                    let email = email.trim();

                    let email = if email.is_empty() {
                        None
                    } else {
                        Some(email.to_string())
                    };

                    println!("\nВведите новый бюджет клиента:");
                    old_value(budget1).await;
                    let mut budget_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut budget_input).unwrap();
                    let budget: i32 = budget_input.trim().parse().expect("Ошибка: бюджет должен быть числом");

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

                    let input2 = from_str_to_int(input_str.trim());

                    match delete_customer(&client, input2.await).await {
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
                } //TODO
            }
        },
        "2" => {
            Ok(())
        },
        "3" => {
            Ok(())
        },
        "4" => {
            Ok(())
        },
        "5" => {
            Ok(())
        },
        _ => {
            input_error().await;
            Ok(())
        }
    }
}