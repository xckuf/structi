use tokio_postgres::{Error, Client};
use std::io::{self, Write};
use crate::order::order::cancel_order;

pub async fn close_order(client: &Client) -> Result<(), Error> {
    let order_id = loop {
        let mut input = String::new();
        println!("Введите id заказа, который нужно закрыть:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(order_id1) => break order_id1,
            Err(_) => println!("\n\nОшибка: id должен быть числом. Пожалуйста, повторите ввод.\n\n")
        }
    };

    match cancel_order(client, order_id).await {
        Ok(message) => println!("{}", message),
        Err(err) => eprintln!("Ошибка при закрытии заказа: {:?}", err),
    }

    Ok(())
}