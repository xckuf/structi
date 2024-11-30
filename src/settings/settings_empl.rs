use tokio_postgres::{Error, Client};
use std::io::{self, Write};
use crate::employee::employee::{delete_employee, get_employee, update_employee};
use crate::models::models::Employee;
use crate::prelude::prelude_valid::*;

pub async fn settings_empl(client: &Client) -> Result<(), Error> {
    let mut input1 = String::new();

    println!("\n\n\n1 - Редактировать работника");
    println!("2 - Удалить работника");
    println!("3 - Выход");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input1).unwrap();

    match input1.as_str().trim() {
        "1" =>{ //Редактировать работника
            let employee_id = loop {
                let mut input = String::new();
                println!("Введите id заказа, данные которой хотите обновить:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(employee_id1) => break employee_id1,
                    Err(_) => println!("\n\nОшибка: id должен быть числом. Пожалуйста, повторите ввод.\n\n"),
                }
            };

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
            old_value(&name1).await;
            let mut name_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut name_input).unwrap();
            let name = if name_input.trim().is_empty() {
                name1.clone()
            } else {
                name_input.trim().to_string()
            };

            println!("\nВведите новую должность сотрудника:");
            old_value(&position1).await;
            let mut position_input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut position_input).unwrap();
            let position = if position_input.trim().is_empty() {
                position1.clone()
            } else {
                position_input.trim().to_string()
            };

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
            old_value(&hire_date1).await;
            let mut hire_date = String::new();
            loop {
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut hire_date).unwrap();
                hire_date = hire_date.trim().to_string();

                if hire_date.is_empty() {
                    hire_date = hire_date1.clone();
                    break;
                }

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

            let mut password = String::new();
            println!("Для удаления работника введите пароль:");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut password).unwrap();
            let password = password.trim();
            if password == "qwerty123" {
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
            } else {
                println!("Неверный пароль. Вы не можете удалить сотрудника");
                Ok(())
            }

            // match get_role(&client, current_id_empl).await {
            //     Ok(true) => {
            //         match delete_employee(&client, input2).await {
            //             Ok(message) => {
            //                 println!("{}", message);
            //                 Ok(())
            //             }
            //             Err(err) => {
            //                 eprintln!("Ошибка при удалении работника: {:?}", err);
            //                 Err(err)
            //             }
            //         }
            //     }
            //     Ok(false) => {
            //         println!("Пока пока: у вас нет прав на удаление сотрудников.");
            //         Ok(())
            //     }
            //     Err(err) => {
            //         eprintln!("Ошибка при проверке роли: {:?}", err);
            //         Err(err)
            //     }
            // }
        },
        "3" => Ok(()),
        _ => {
            input_error().await;
            Ok(())
        }
    }
}