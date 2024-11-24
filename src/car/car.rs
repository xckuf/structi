use crate::models::{ Car };
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
        "CREATE TABLE IF NOT EXISTS car (
            id SERIAL PRIMARY KEY,
            brand VARCHAR(100) NOT NULL,
            model VARCHAR(100) NOT NULL,
            year VARCHAR(20) NOT NULL,
            price INTEGER NOT NULL,
            mileage REAL NOT NULL,
            is_new BOOLEAN NOT NULL DEFAULT FALSE
        )",
        &[],
    ).await?;

    Ok(client)
}

pub async fn create_car(client: &Client, car: Car) -> Result<i32, Error> {
    let row = client.query_one(
        "INSERT INTO car (brand, model, year, price, mileage, is_new) VALUES ($2, $3, $4, $5)",
        &[&car.brand, &car.model, &car.year, &car.price, &car.mileage, &car.is_new],
    ).await?;
    Ok(row.get(0))
}

pub async fn update_car(client: &Client, car_id: i32, updated_car: Car) -> Result<(), Error> {
    client.execute(
        "UPDATE car SET brand = $2, model = $3, year = $4, price = $5, mileage = $6, is_new = $7 WHERE id = $1",
        &[&updated_car.brand, &updated_car.model, &updated_car.year, &updated_car.price, &updated_car.mileage, &updated_car.is_new, &car_id],
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