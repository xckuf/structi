use std::io::{self};
use tokio_postgres::{ Error, Client };
use crate::models::models::{Customer};
use crate::customer::customer::{get_all_customers, search_customer};
use crate::prelude::prelude_valid::*;

pub async fn choose_customer(client: &Client, current_id_cust: &mut i32) -> Result<Option<Customer>, Error> {
    let mut choice = String::new(); //TODO
    println!("\n\n\n1 - Вывести всех клиентов");
    println!("2 - Найти клиента по фильтрам");

    io::stdin().read_line(&mut choice).expect("Ошибка");

    match choice.as_str().trim() {
        "1" => { //вывести всех клиентов
            match get_all_customers(&client).await {
                Ok(customers) => {
                    if customers.is_empty() {
                        println!("Клиенты отсутствуют.");
                        return Ok(None);
                    }
                    for customer in &customers {
                        println!("\n\nid: {}", &customer.id.unwrap_or(0));
                        println!("Имя: {}", &customer.name);
                        println!("Телефон: {}", &customer.phone);
                        println!(
                            "email: {}",
                            customer.email.clone().unwrap_or_else(|| "Не указан".to_string())
                        );
                        println!("Бюджет: {}\n\n", customer.budget);
                    }

                    println!("\n\n\nВведите id клиента для выбора:");
                    let mut id_input = String::new();
                    io::stdin().read_line(&mut id_input).unwrap();
                    let id = id_input.trim().parse::<i32>().ok();


                    if let Some(id) = id {
                        if let Some(customer) = customers.into_iter().find(|c| c.id == Some(id)) {
                            *current_id_cust = id;
                            Ok(Some(customer))
                        } else {
                            println!("Клиент с таким id не найден.");
                            Ok(None)
                        }
                    } else {
                        println!("Некорректный ввод id.");
                        Ok(None)
                    }
                }
                Err(err) => {
                    eprintln!("Ошибка при получении клиентов: {:?}", err);
                    Err(err)
                }
            }
        },
        "2" => { //клиент по фильтрам
            let mut input = String::new();

            let mut id1: Option<i32> = None;
            let mut name1: Option<String> = None;
            let mut phone1: Option<String> = None;
            let mut email1: Option<String> = None;
            let mut budget1: Option<i32> = None;
            loop {
                println!("\n\n\n1) id: {:?}", id1);
                println!("2) name: {:?}", name1);
                println!("3) phone: {:?}", phone1);
                println!("4) email: {:?}", email1);
                println!("5) budget: {:?}", budget1);
                println!("6) Готово");

                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                match input.as_str().trim() {
                    "1" => {
                        let mut id_str = String::new();
                        println!("Введите id:");
                        io::stdin().read_line(&mut id_str).unwrap();
                        id1 = id_str.trim().parse::<i32>().ok();
                    },
                    "2" => {
                        let mut name_str = String::new();
                        println!("Введите имя:");
                        io::stdin().read_line(&mut name_str).unwrap();
                        name1 = Some(name_str.trim().to_string());
                    },
                    "3" => {
                        let mut phone_str = String::new();
                        println!("Введите телефон:");
                        io::stdin().read_line(&mut phone_str).unwrap();
                        phone1 = Some(phone_str.trim().to_string());
                    },
                    "4" => {
                        let mut email_str = String::new();
                        println!("Введите email:");
                        io::stdin().read_line(&mut email_str).unwrap();
                        email1 = Some(email_str.trim().to_string());
                    }
                    "5" => {
                        let mut budget_str = String::new();
                        println!("Введите бюджет:");
                        io::stdin().read_line(&mut budget_str).unwrap();
                        budget1 = budget_str.trim().parse::<i32>().ok();
                    },
                    "6" => break,
                    _ => println!("Неверный ввод")
                }
            }
            match search_customer(&client, id1, name1, phone1, email1, budget1).await {
                Ok(customers) => {
                    if customers.is_empty() {
                        println!("Клиенты не найдены.");
                        return Ok(None);
                    } else {
                        for customer in &customers {
                            println!("{}", customer.pretty_print_customers());
                        }

                        println!("\n\n\nВведите id клиента для выбора:");
                        let mut id_input = String::new();
                        io::stdin().read_line(&mut id_input).unwrap();
                        let id = id_input.trim().parse::<i32>().ok();
                        *current_id_cust = id.clone().unwrap();

                        if let Some(id) = id {
                            if let Some(customer) = customers.into_iter().find(|c| c.id == Some(id)) {
                                return Ok(Some(customer));
                            } else {
                                println!("Клиент с таким id не найден.");
                            }
                        } else {
                            println!("Некорректный ввод id.");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Ошибка при поиске клиентов: {:?}", err);
                    return Err(err)
                }
            }
            Ok(None)
        },
        _ => {
            input_error().await;
            Ok(None)
        }
    }
}