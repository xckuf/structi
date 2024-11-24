#[derive(Debug)]
pub struct Car {
    pub id: Option<i32>,
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub price: i32,
    pub mileage: i32,
    pub is_new: bool,
}
#[derive(Debug)]
pub struct Customer {
    pub id: Option<i32>,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub budget: i32,
}

#[derive(Debug)]
pub struct Employee {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub salary: i32,
    pub hire_date: String,
}

#[derive(Debug)]
pub struct Order {
    pub id: Option<i32>,
    pub car_id: i32,
    pub customer_id: i32,
    pub employee_id: i32,
    pub order_date: String,
    pub price: i32,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct Dealership {
    pub id: Option<i32>,
    pub name: String,
    pub location: String,
    pub employees: Vec<Employee>,
    pub cars: Vec<Car>,
    pub orders: Vec<Order>,
}
