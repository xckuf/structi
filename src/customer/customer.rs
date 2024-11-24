use crate::models::models::Customer;
use tokio_postgres::{ NoTls, Error, Client };
use dotenv::dotenv;
use std::env;

async fn connect() -> Result<Client, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

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
        "INSERT INTO customer (name, phone, email, budget) VALUES ($1, $2, $3, $4) RETURNING id",
        &[&customer.name, &customer.phone, &customer.email, &customer.budget],
    ).await?;
    Ok(row.get(0))
}

pub async fn update_customer(client: &Client, customer_id: i32, updated_customer: Customer) -> Result<String, Error> {
    let result = client.execute(
        "UPDATE customer SET name = $2, phone = $3, email = $4, budget = $5 WHERE id = $1",
        &[&customer_id, &updated_customer.name, &updated_customer.phone, &updated_customer.email, &updated_customer.budget],
    ).await?;
    if result > 0 {
        Ok("Изменения успешно сохранены".to_string())
    } else {
        Ok("Изменения не применены".to_string())
    }
}

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

pub async fn get_all_customers(client: &Client) -> Result<Vec<Customer>, Error> {
    let rows = client.query(
        "SELECT id, name, phone, email, budget FROM customer",
        &[],
    ).await?;

    let customers = rows.iter().map(|row| Customer {
        id: row.get(0),
        name: row.get(1),
        phone: row.get(2),
        email: row.get(3),
        budget: row.get(4),
    }).collect();

    Ok(customers)
}

pub async fn search_customer(
    client: &Client,
    id: Option<i32>,
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    budget: Option<i32>,
) -> Result<Vec<Customer>, Error> {
    let mut query = String::from(
        "SELECT id, name, phone, email, budget FROM customer WHERE 1=1"
    );
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

    let mut id1 = None;
    let mut name1 = String::new();
    let mut phone1 = String::new();
    let mut email1 = String::new();
    let mut budget1 = None;

   if let Some(id) = id {
       query.push_str(&format!(" AND id = ${}", params.len() + 1));
       id1 = Some(id);
       params.push(&id1);
   }

    if let Some(name) = name.as_deref() {
        query.push_str(&format!(" AND name = ${}", params.len() + 1));
        name1 = name.to_string();
        params.push(&name1);
    }

    if let Some(phone) = phone.as_deref() {
        query.push_str(&format!(" AND phone = ${}", params.len() + 1));
        phone1 = phone.to_string();
        params.push(&phone1);
    }

    if let Some(email) = email.as_deref() {
        query.push_str(&format!(" AND email = ${}", params.len() + 1));
        email1 = email.to_string();
        params.push(&email1);
    }

    if let Some(budget) = budget {
        query.push_str(&format!(" AND budget = ${}", params.len() + 1));
        budget1 = Some(budget);
        params.push(&budget1);
    }

    let rows = client.query(&query, &params).await?;

    let customers = rows
        .iter()
        .map(|row| Customer {
            id: row.get(0),
            name: row.get(1),
            phone: row.get(2),
            email: row.get(3),
            budget: row.get(4),
        })
        .collect();

    Ok(customers)
}