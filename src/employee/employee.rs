use crate::models::models::Employee;
use tokio_postgres::{ Error, Client };

// async fn connect() -> Result<Client, Error> {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
//     let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
//
//     // Запускаем соединение в фоновом режиме
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("Ошибка подключения к базе данных: {}", e);
//         }
//     });
//
//     client.execute(
//         "CREATE TABLE IF NOT EXISTS employee (
//             id SERIAL PRIMARY KEY,
//             name VARCHAR(100) NOT NULL,
//             position VARCHAR(50) NOT NULL,
//             salary INTEGER NOT NULL,
//             hire_date DATE NOT NULL DEFAULT CURRENT_DATE
//         )",
//         &[],
//     ).await?;
//
//     Ok(client)
// }


pub async fn create_employee(client: &Client, employee: Employee) -> Result<i32, Error> {
    let row = client.query_one(
        "INSERT INTO employee (name, position, salary, hire_date) VALUES ($1, $2, $3, $4) RETURNING id",
        &[&employee.name, &employee.position, &employee.salary, &employee.hire_date],
    ).await?;
    Ok(row.get(0))
}

pub async fn update_employee(client: &Client, employee_id: i32, updated_employee: Employee) -> Result<String, Error> {
    let result = client.execute(
        "UPDATE employee SET name = $2, position = $3, salary = $4, hire_date = $5 WHERE id = $1",
        &[&employee_id, &updated_employee.name, &updated_employee.position, &updated_employee.salary, &updated_employee.hire_date],
    ).await?;
    if result > 0 {
        Ok("\n\n\nИзменения успешно сохранены".to_string())
    } else {
        Ok("\n\n\nИзменения не применены".to_string())
    }
}

pub async fn delete_employee(client: &Client, employee_id: i32) -> Result<String, Error> {
    let result = client.execute(
        "DELETE FROM employee WHERE id = $1",
        &[&employee_id],
    ).await?;

    if result > 0 {
        Ok("\n\n\nИзменения успешно сохранены".to_string())
    } else {
        Ok("\n\n\nУдаление не выполнено".to_string())
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

// pub async fn search_employees(
//     client: &Client,
//     name: Option<String>,
//     position: Option<String>,
//     salary: Option<i32>,
//     hire_date: Option<String>,
// ) -> Result<Vec<Employee>, Error> {
//     let mut query = String::from(
//         "SELECT id, name, position, salary, hire_date from employee WHERE 1=1"
//     );
//     let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
//
//     let mut name1 = String::new();
//     let mut position1 = String::new();
//     let mut salary1 = None;
//     let mut hire_date1 = String::new();
//
//     if let Some(name) = name.as_deref() {
//         query.push_str(&format!(" AND name = ${}", params.len() + 1));
//         name1 = name.to_string();
//         params.push(&name1);
//     }
//
//     if let Some(position) = position.as_deref() {
//         query.push_str(&format!(" AND position = ${}", params.len() + 1));
//         position1 = position.to_string();
//         params.push(&position1);
//     }
//
//     if let Some(salary) = salary {
//         query.push_str(&format!(" AND salary = ${}", params.len() + 1));
//         salary1 = Some(salary);
//         params.push(&salary1);
//     }
//     if let Some(hire_date) = hire_date.as_deref() {
//         query.push_str(&format!(" AND hire date = ${}", params.len() + 1));
//         hire_date1 = hire_date.to_string();
//         params.push(&hire_date1);
//     }
//
//     let rows = client.query(&query, &params).await?;
//
//     let employees = rows
//         .iter()
//         .map(|row| Employee {
//             id: row.get(0),
//             name: row.get(1),
//             position: row.get(2),
//             salary: row.get(3),
//             hire_date: row.get(4),
//         })
//         .collect();
//
//     Ok(employees)
// }