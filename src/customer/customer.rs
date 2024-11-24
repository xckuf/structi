use crate::models::{ Customer };
use tokio_postgres::{ NoTls, Error, Client };

async fn connect() -> Result<Client, Error> {
    let connection_str = "host=localhost user=postgres password=86245Qaz dbname=dealership";
    let (client, connection) = tokio_postgres::connect(connection_str, NoTls).await?;

    // Запускаем соединение в фоновом режиме
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Ошибка подключения к базе данных: {}", e);
        }
    });

    client.execute(
        "CREATE TABLE IF NOT EXISTS customer (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            phone VARCHAR(50) NOT NULL,
            email VARCHAR(100) ,
            budget INTEGER NOT NULL
        )",
        &[],
    ).await?;

    Ok(client)
}

pub async fn create_customer(client: &Client, customer: Customer) -> Result<i32, Error> {
    let row = client.query_one(
        "INSERT INTO customer (name, phone, email, budget) VALUES ($2, $3, $4, $5)",
        &[&customer.name, &customer.phone, &customer.email, &customer.budget],
    ).await?;
    Ok(row.get(0))
}

pub async fn update_customer(client: &Client, customer_id: i32, updated_customer: Customer) -> Result<String, Error> {
    let result = client.execute(
        "UPDATE customer SET name = $2, phone = $3, email = $4, budget = $5 WHERE id = $1",
        &[&updated_customer.name, &updated_customer.phone, &updated_customer.email, &updated_customer.budget, &customer_id],
    ).await?;
    if result > 0 {
        Ok("Изменения успешно сохранены".to_string())
    } else {
        Ok("Изменения не применены".to_string())
    }
}

// Удаление сотрудника
pub async fn delete_customer(client: &Client, customer_id: i32) -> Result<String, Error> {
    let result = client.execute(
        "DELETE FROM customer WHERE id = $1",
        &[&customer_id],
    ).await?;

    if result > 0 {
        Ok("Изменения успешно сохранены".to_string())
    } else {
        Ok("Удаление не выполнено".to_string())
    }
}

pub async fn get_customer(client: &Client, customer_id: i32) -> Result<Option<Customer>, Error> {
    let row = client.query_opt(
        "SELECT id, name, phone, email, budget FROM customer WHERE id = $1",
        &[&customer_id],
    ).await?;

    if let Some(row) = row {
        Ok(Some(Customer {
            id: row.get(0),
            name: row.get(1),
            phone: row.get(2),
            email: row.get(3),
            budget: row.get(4),
        }))
    } else {
        Ok(None)
    }
}