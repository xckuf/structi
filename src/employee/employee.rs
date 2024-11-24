use crate::models::Employee;
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
        "CREATE TABLE IF NOT EXISTS employee (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            position VARCHAR(50) NOT NULL,
            salary INTEGER NOT NULL,
            hire_date VARCHAR(50) NOT NULL
        )",
        &[],
    ).await?;

    Ok(client)
}

// Создание нового сотрудника
pub async fn create_employee(client: &Client, employee: Employee) -> Result<i32, Error> {
    let row = client.query_one(
        "INSERT INTO employee (name, position, salary, hire_date) VALUES ($1, $2, $3, $4) RETURNING id",
        &[&employee.name, &employee.position, &employee.salary, &employee.hire_date],
    ).await?;
    Ok(row.get(0))
}

// Обновление информации о сотруднике
pub async fn update_employee(client: &Client, employee_id: i32, updated_employee: Employee) -> Result<String, Error> {
    let result = client.execute(
        "UPDATE employee SET name = $2, position = $3, salary = $4, hire_date = $5 WHERE id = $1",
        &[&updated_employee.name, &updated_employee.position, &updated_employee.salary, &updated_employee.hire_date, &employee_id],
    ).await?;
    if result > 0 {
        Ok("Изменения успешно сохранены".to_string())
    } else {
        Ok("Изменения не применены".to_string())
    }
}

// Удаление сотрудника
pub async fn delete_employee(client: &Client, employee_id: String) -> Result<String, Error> {
    let result = client.execute(
        "DELETE FROM employees WHERE id = $1",
        &[&employee_id],
    ).await?;

    if result > 0 {
        Ok("Изменения успешно сохранены".to_string())
    } else {
        Ok("Удаление не выполнено".to_string())
    }
}

pub async fn get_employee(client: &Client, employee_id: i32) -> Result<Option<Employee>, Error> {
    let row = client.query_opt(
        "SELECT id, name, position, salary, hire_date FROM employee WHERE id = $1",
        &[&employee_id],
    ).await?;

    if let Some(row) = row {
        Ok(Some(Employee {
            id: row.get(0),
            name: row.get(1),
            position: row.get(2),
            salary: row.get(3),
            hire_date: row.get(4),
        }))
    } else {
        Ok(None)
    }
}