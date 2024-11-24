use crate::models::models::Car;
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
        "CREATE TABLE IF NOT EXISTS car (
            id SERIAL PRIMARY KEY,
            brand VARCHAR(100) NOT NULL,
            model VARCHAR(100) NOT NULL,
            year INTEGER NOT NULL,
            price INTEGER NOT NULL,
            mileage INTEGER NOT NULL,
            is_new BOOLEAN NOT NULL DEFAULT FALSE
        )",
        &[],
    ).await?;

    Ok(client)
}

pub async fn create_car(client: &Client, car: Car) -> Result<i32, Error> {
    let row = client.query_one(
        "INSERT INTO car (brand, model, year, price, mileage, is_new) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        &[&car.brand, &car.model, &car.year, &car.price, &car.mileage, &car.is_new],
    ).await?;
    Ok(row.get(0))
}

pub async fn update_car(client: &Client, car_id: i32, updated_car: Car) -> Result<(), Error> {
    client.execute(
        "UPDATE car SET brand = $2, model = $3, year = $4, price = $5, mileage = $6, is_new = $7 WHERE id = $1",
        &[&car_id, &updated_car.brand, &updated_car.model, &updated_car.year, &updated_car.price, &updated_car.mileage, &updated_car.is_new],
    ).await?;
    Ok(())
}

pub async fn delete_car(client: &Client, car_id: i32) -> Result<(), Error> {
    client.execute(
        "DELETE FROM car WHERE id = $1",
        &[&car_id],
    ).await?;
    Ok(())
}

pub async fn get_car(client: &Client, car_id: i32) -> Result<Option<Car>, Error> {
    let row = client.query_opt(
        "SELECT id, brand, model, year, price, mileage, is_new FROM car WHERE id = $1",
        &[&car_id],
    ).await?;

    if let Some(row) = row {
        Ok(Some(Car {
            id: row.get(0),
            brand: row.get(1),
            model: row.get(2),
            year: row.get(3),
            price: row.get(4),
            mileage: row.get(5),
            is_new: row.get(6),
        }))
    } else {
        Ok(None)
    }
}

pub async fn budget_car_do(client: &Client, budget: i32) -> Result<Vec<Car>, Error> {
    let query = "SELECT id, brand, model, year, price, mileage, is_new FROM car WHERE price <= $1";

    let rows = client.query(query, &[&budget]).await?;

    let cars = rows
        .iter()
        .map(|row| Car {
            id: row.get(0),
            brand: row.get(1),
            model: row.get(2),
            year: row.get(3),
            price: row.get(4),
            mileage: row.get(5),
            is_new: row.get(6),
        })
        .collect();

    Ok(cars)
}

pub async fn budget_car_ot(client: &Client, budget: i32) -> Result<Vec<Car>, Error> {
    let query = "SELECT id, brand, model, year, price, mileage, is_new FROM car WHERE price >= $1";

    let rows = client.query(query, &[&budget]).await?;

    let cars = rows
        .iter()
        .map(|row| Car {
            id: row.get(0),
            brand: row.get(1),
            model: row.get(2),
            year: row.get(3),
            price: row.get(4),
            mileage: row.get(5),
            is_new: row.get(6),
        })
        .collect();

    Ok(cars)
}

pub async fn search_cars(
    client: &Client,
    id: Option<i32>,
    brand: Option<String>,
    model: Option<String>,
    year: Option<i32>,
    price: Option<i32>,
    mileage: Option<i32>,
    is_new: Option<bool>,
) -> Result<Vec<Car>, Error> {
    let mut query = String::from(
        "SELECT id, brand, model, year, price, mileage, is_new FROM car WHERE 1=1"
    );
    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

    let mut brand1 = String::new();
    let mut model1 = String::new();
    let mut year1 = None;
    let mut price1 = None;
    let mut mileage1 = None;
    let mut is_new1 = None;


    if let Some(brand) = brand {
        query.push_str(&format!(" AND brand = ${}", params.len() + 1));
        brand1 = brand.to_string();
        params.push(&brand1);
    }

    if let Some(model) = model {
        query.push_str(&format!(" AND model = ${}", params.len() + 1));
        model1 = model.to_string();
        params.push(&model1);
    }

    if let Some(year) = year {
        query.push_str(&format!(" AND year = ${}", params.len() + 1));
        year1 = Some(year);
        params.push(&year1);
    }

    if let Some(price) = price {
        query.push_str(&format!(" AND price = ${}", params.len() + 1));
        price1 = Some(price);
        params.push(&price1);
    }

    if let Some(mileage) = mileage {
        query.push_str(&format!(" AND mileage = ${}", params.len() + 1));
        mileage1 = Some(mileage);
        params.push(&mileage1);
    }

    if let Some(is_new) = is_new {
        query.push_str(&format!(" AND is_new = ${}", params.len() + 1));
        is_new1 = Some(is_new);
        params.push(&is_new1);
    }

    let rows = client.query(&query, &params).await?;

    let cars = rows
        .iter()
        .map(|row| Car {
            id: row.get(0),
            brand: row.get(1),
            model: row.get(2),
            year: row.get(3),
            price: row.get(4),
            mileage: row.get(5),
            is_new: row.get(6),
        })
        .collect();

    Ok(cars)
}