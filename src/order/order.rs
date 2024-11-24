use tokio_postgres::{Client, Error, NoTls};
use crate::models::models::Order;
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
        "CREATE TABLE IF NOT EXISTS orders (
            id SERIAL PRIMARY KEY,
            car_id INTEGER NOT NULL,
            customer_id INTEGER NOT NULL,
            employee_id INTEGER NOT NULL,
            order_date VARCHAR(20) NOT NULL ,
            price INTEGER NOT NULL,
            is_active BOOLEAN DEFAULT true
        )",
        &[],
    ).await?;

    Ok(client)
}

pub async fn create_order(client: &Client, order: Order) -> Result<i32, Error> {
    let row = client.query_one(
        "INSERT INTO orders (car_id, customer_id, employee_id, order_date, price, is_active)\
         VALUES ($1, $2, $3, $4, $5, $6)\
         RETURNING id",
        &[&order.car_id, &order.customer_id, &order.employee_id, &order.order_date.to_string(), &order.price, &order.is_active],
    ).await?;
    Ok(row.get(0))
}

pub async fn update_order(client: &Client, order_id: i32, updated_order: Order) -> Result<String, Error> {
    let result = client.execute(
        "UPDATE orders SET car_id = $2, customer_id = $3, employee_id = $4, \
                order_date = $5, price = $6, is_active = $7 WHERE id = $1",
        &[&order_id, &updated_order.car_id, &updated_order.customer_id, &updated_order.employee_id, &updated_order.order_date.to_string(), &updated_order.price, &updated_order.is_active],
    ).await?;

    if result > 0 {
        Ok("Изменения успешно сохранены".to_string())
    } else {
        Ok("Изменения не применены".to_string())
    }
}

pub async fn delete_order(client: &Client, order_id: i32) -> Result<String, Error> {
    let result = client.execute(
        "DELETE FROM orders WHERE id = $1",
        &[&order_id],
    ).await?;

    if result > 0 {
        Ok("Изменения успешно сохранены".to_string())
    } else {
        Ok("Удаление не выполнено".to_string())
    }
}

pub async fn get_order(client: &Client, order_id: i32) -> Result<Option<Order>, Error> {
    let row = client.query_opt(
        "SELECT id, car_id, customer_id, employee_id, order_date, price, is_active FROM orders WHERE id = $1",
        &[&order_id],
    ).await?;

    if let Some(row) = row {
        Ok(Some(Order {
            id: row.get(0),
            car_id: row.get(1),
            customer_id: row.get(2),
            employee_id: row.get(3),
            order_date: row.get(4),
            price: row.get(5),
            is_active: row.get(6),
        }))
    } else {
        Ok(None)
    }
}

pub async fn cancel_order(client: &Client, order_id: i32) -> Result<String, Error> {
    let result = client.execute(
        "UPDATE orders SET is_active = false WHERE id = $1",
        &[&order_id],
    ).await?;

    if result > 0 {
        Ok("Заказ успешно закрыт".to_string())
    } else {
        Ok("Закрытие заказа не выполнено".to_string())
    }
}

pub async fn search_orders(
    client: &Client,
    car_id: Option<i32>,
    customer_id: Option<i32>,
    employee_id: Option<i32>,
    order_date: Option<String>,
    price: Option<i32>,
    is_active: Option<bool>,
) -> Result<Vec<Order>, Error> {
    let mut query = String::from(
        "SELECT id, car_id, customer_id, employee_id, order_date, price, is_active FROM orders WHERE 1=1",
    );
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

    let mut car1 = None;
    let mut customer1 = None;
    let mut employee1 = None;
    let mut order_date1 = String::new();
    let mut price1 = None;
    let mut is_active1 = None;


    if let Some(car) = car_id {
        query.push_str(&format!(" AND car_id = ${}", params.len() + 1));
        car1 = Some(car);
        params.push(&car1);
    }
    if let Some(customer) = customer_id {
        query.push_str(&format!(" AND customer_id = ${}", params.len() + 1));
        customer1 = Some(customer);
        params.push(&customer1);
    }
    if let Some(employee) = employee_id {
        query.push_str(&format!(" AND employee_id = ${}", params.len() + 1));
        employee1 = Some(employee);
        params.push(&employee1);
    }
    if let Some(date) = order_date {
        query.push_str(&format!(" AND order_date = ${}", params.len() + 1));
        order_date1 = date.to_string();
        params.push(&order_date1);
    }
    if let Some(order_price) = price {
        query.push_str(&format!(" AND price = ${}", params.len() + 1));
        price1 = Some(order_price);
        params.push(&price1);
    }
    if let Some(active) = is_active {
        query.push_str(&format!(" AND is_active = ${}", params.len() + 1));
        is_active1 = Some(active);
        params.push(&is_active1);
    }

    let rows = client.query(&query, &params).await?;

    let orders = rows
        .iter()
        .map(|row| Order {
            id: row.get("id"),
            car_id: row.get("car_id"),
            customer_id: row.get("customer_id"),
            employee_id: row.get("employee_id"),
            order_date: row.get("order_date"),
            price: row.get("price"),
            is_active: row.get("is_active"),
        })
        .collect();

    Ok(orders)
}
