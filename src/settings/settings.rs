use tokio_postgres::{Error, Client};
use std::io::{self, Write};
use crate::settings::settings_car::settings_car;
use crate::settings::settings_cust::settings_cust;
use crate::settings::settings_empl::settings_empl;
use crate::settings::settings_order::settings_order;
use crate::validation::input_error::input_error;

pub async fn settings(client: &Client, employee_id: i32) -> Result<(), Error> {
    let mut input = String::new();

    println!("\n\n\n1 - Клиент");
    println!("2 - Работник");
    println!("3 - Автомобиль");
    println!("4 - Заказ");
    println!("5 - Выход");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    match input.as_str().trim() {
        "1" => { //Клиент
            settings_cust(&client).await
        },
        "2" => {//Работник
            settings_empl(&client, employee_id).await
        },
        "3" => { //Автомобиль
            settings_car(&client).await
        },
        "4" => { //Заказ
            settings_order(&client).await
        },
        "5" => { //Выход
            Ok(())
        },
        _ => {
            input_error().await;
            Ok(())
        }
    }
}