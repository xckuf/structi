
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

impl Car {
    pub fn pretty_print_car(&self) -> String {
        format!(
            "\n\n\nid: {}\nbrand: {}\nmodel: {}\nyear: {}\nprice: {}\nmileage: {}\nis_new: {}",
            self.id.unwrap_or_default(),
            self.brand,
            self.model,
            self.year,
            self.price,
            self.mileage,
            if self.is_new { "Новый" } else { "Не новый" }
        )
    }
}

#[derive(Debug)]
pub struct Customer {
    pub id: Option<i32>,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub budget: i32,
}

impl Customer {
    pub fn pretty_print_customers(&self) -> String {
        format!(
            "\n\n\nid: {}\nname: {}\nphone: {}\nemail: {}\nbudget: {}",
            self.id.unwrap_or_default(),
            self.name,
            self.phone,
            self.email.clone().unwrap_or_else(|| String::from("Не указан")),
            self.budget
        )
    }
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

// impl Order {
//     pub fn pretty_print_orders(&self) -> String {
//         format!(
//             "\n\n\nid: {}\ncar_id: {}\ncustomer_id: {}\nemployee_id: {}\norder_date: {}\nprice: {}\nis_active: {}\n",
//             self.id.unwrap_or_default(),
//             self.car_id,
//             self.customer_id,
//             self.employee_id,
//             self.order_date,
//             self.price,
//             self.is_active
//         )
//     }
// }

// #[derive(Debug)]
// pub struct Dealership {
//     pub id: Option<i32>,
//     pub name: String,
//     pub location: String,
//     pub employees: Vec<Employee>,
//     pub cars: Vec<Car>,
//     pub orders: Vec<Order>,
// }
