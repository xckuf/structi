use tokio_postgres::{Error, Client};
use tokio;
use std::io::{self, Write};
use std::option::Option;
use crate::customer::customer::{update_customer, get_customer, delete_customer};
use crate::employee::employee::{delete_employee, get_employee, update_employee};
use crate::car::car::{delete_car, get_car, update_car};
use crate::models::models::{Car, Customer, Employee};

use crate::validation::from_str_to_int::from_str_to_int;
use crate::validation::input_error::input_error;
use crate::validation::old_value::old_value;
use crate::validation::valid_date::valid_date;

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
                } //TODO
            }
        },
        "2" => {//Работник
            let mut input1 = String::new();

            println!("1 - Редактировать работника");
            println!("2 - Удалить работника");
            println!("3 - Выход");

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input1).unwrap();

            match input1.as_str().trim() {
                "1" =>{ //Редактировать работника
                    println!("Введите ID сотрудника, данные которого хотите обновить:");
                    let mut employee_id_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut employee_id_input).unwrap();
                    let employee_id: i32 = employee_id_input.trim().parse().expect("Ошибка: ID должен быть числом");

                    let mut name1 = String::new();
                    let mut position1 = String::new();
                    let mut salary1: i32 = 0;
                    let mut hire_date1 = String::new();

                    match get_employee(&client, employee_id).await {
                        Ok(Some(employee)) => {
                            name1 = employee.name;
                            position1 = employee.position;
                            salary1 = employee.salary;
                            hire_date1 = employee.hire_date;
                        }
                        Ok(None) => {
                            println!("Работник с ID {} не найден.", employee_id);
                        }
                        Err(err) => {
                            eprintln!("Ошибка при получении работника: {:?}", err);
                        }
                    }

                    println!("\nВведите новое имя сотрудника:");
                    old_value(name1).await;
                    let mut name = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut name).unwrap();
                    let name = name.trim().to_string();

                    println!("\nВведите новую должность сотрудника:");
                    old_value(position1).await;
                    let mut position = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut position).unwrap();
                    let position = position.trim().to_string();

                    println!("\nВведите новую зарплату сотрудника:");
                    old_value(salary1).await;
                    let mut salary_str = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut salary_str).unwrap();
                    let salary: i32 = if salary_str.trim().is_empty() {
                        salary1
                    } else {
                        salary_str
                            .trim()
                            .parse()
                            .unwrap_or_else(|_| {
                                eprintln!("Ошибка: зарплата должна быть числом. Используем старое значение.");
                                salary1
                            })
                    };

                    println!("\nВведите новую дату, когда устроился на работу:");
                    old_value(hire_date1).await;
                    let mut hire_date = String::new();
                    loop {
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut hire_date).unwrap();
                        hire_date = hire_date.trim().to_string();
                        match valid_date(hire_date.clone()).await {
                            Ok(_) => break,
                            Err(err) => {
                                println!("Ошибка: {}", err);
                                println!("Попробуйте снова. Введите дату в формате дд.мм.гггг:");
                                hire_date.clear();
                            }
                        }
                    }

                    let updated_employee = Employee {
                        id: Some(employee_id),
                        name,
                        position,
                        salary,
                        hire_date,
                    };

                    match update_employee(&client, employee_id, updated_employee).await {
                        Ok(message) => {
                            println!("{}", message);
                            Ok(())
                        },
                        Err(err) => {
                            eprintln!("Ошибка при обновлении клиента: {:?}", err);
                            Err(err)
                        }
                    }
                },
                "2" => { //Удалить работника
                    let mut input_str = String::new();

                    println!("Введите id работника:");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input_str).unwrap();

                    let input2 = from_str_to_int(input_str.trim()).await;

                    match delete_employee(&client, input2).await {
                        Ok(message) => {
                            println!("{}", message);
                            Ok(())
                        },
                        Err(err) => {
                            eprintln!("Ошибка при удалении работника: {:?}", err);
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
        },
        "3" => { //Автомобиль
            let mut input1 = String::new();

            println!("1 - Редактировать машину");
            println!("2 - Удалить машину");
            println!("3 - Выход");

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input1).unwrap();

            match input1.as_str().trim() {
                "1" => {
                    println!("Введите ID машины, данные которой хотите обновить:");
                    let mut car_id_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut car_id_input).unwrap();
                    let car_id: i32 = car_id_input.trim().parse().expect("Ошибка: ID должен быть числом");

                    let mut brand1 = String::new();
                    let mut model1 = String::new();
                    let mut year1: i32 = 0;
                    let mut price1: i32 = 0;
                    let mut mileage1: i32 = 0;
                    let mut is_new1 = false;

                    match get_car(&client, car_id).await {
                        Ok(Some(car)) => {
                            brand1 = car.brand;
                            model1 = car.model;
                            year1 = car.year;
                            price1 = car.price;
                            mileage1 = car.mileage;
                            is_new1 = car.is_new;
                        }
                        Ok(None) => {
                            println!("Машина с ID {} не найден.", car_id);
                        }
                        Err(err) => {
                            eprintln!("Ошибка при получении машины: {:?}", err);
                        }
                    }

                    println!("\nВведите новую марку автомобиля:");
                    old_value(brand1).await;
                    let mut brand = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut brand).unwrap();
                    let brand = brand.trim().to_string();

                    println!("\nВведите новую модель автомобиля:");
                    old_value(model1).await;
                    let mut model = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut model).unwrap();
                    let model = model.trim().to_string();

                    println!("\nВведите новый год автомобиля:");
                    old_value(year1).await;
                    let mut year_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut year_input).unwrap();
                    let year = year_input.trim().parse().expect("Ошибка: год должен быть числом");

                    println!("\nВведите новую цену автомобиля:");
                    old_value(price1).await;
                    let mut price_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut price_input).unwrap();
                    let price = price_input.trim().parse().expect("Ошибка: цена должен быть числом");

                    println!("\nВведите новый пробег автомобиля:");
                    old_value(mileage1).await;
                    let mut mileage_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut mileage_input).unwrap();
                    let mileage = mileage_input.trim().parse().expect("Ошибка: пробег должен быть числом");

                    println!("\nВведите новое состояние автомобиля: \n\
                    0 (Новый автомобиль) или 1 (Не новый автомобиль)");
                    old_value(is_new1).await;
                    let mut is_new_input = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut is_new_input).unwrap();
                    match is_new_input.trim() {
                        "0" => is_new1 = true,
                        "1" => is_new1 = false,
                        _ => {
                            println!("Неверный ввод. Введите 0 или 1.");
                        }
                    }
                    let is_new = is_new1;

                    let updated_car = Car {
                        id: Some(car_id),
                        brand,
                        model,
                        year,
                        price,
                        mileage,
                        is_new
                    };

                    match update_car(&client, car_id, updated_car).await {
                        Ok(message) => {
                            println!("{}", message);
                            Ok(())
                        },
                        Err(err) => {
                            eprintln!("Ошибка при обновлении машины: {:?}", err);
                            Err(err)
                        }
                    }
                },
                "2" => {//Удалить автомобиль
                    let mut input_str = String::new();

                    println!("Введите id машины:");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input_str).unwrap();

                    let input2 = from_str_to_int(input_str.trim());

                    match delete_car(&client, input2.await).await {
                        Ok(message) => {
                            println!("{}", message);
                            Ok(())
                        },
                        Err(err) => {
                            eprintln!("Ошибка при удалении автомобиля: {:?}", err);
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
        },
        "4" => { //Заказ
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