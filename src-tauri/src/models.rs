use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct Product {
    pub id: i32,
    pub upc: String,
    pub name: String,
    pub description: String,
    pub amount: f64,
    pub case_size: Option<i32>,
    pub measure_by_weight: bool,
    pub cost_price_per_unit: BigDecimal,
    pub selling_price_per_unit: BigDecimal,
    pub sale_end: Option<NaiveDateTime>,
    pub buy_level: Option<f64>,
    pub sale_price: Option<BigDecimal>,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone)]
pub struct Preference {
    pub user_id: i32,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone)]
pub struct Permission {
    pub user_id: i32,
    pub admin: bool,
    pub view_pending: bool,
    pub view_received: bool,
    pub edit_pending: bool,
    pub create_orders: bool,
    pub edit_received: bool,
    pub remove_orders: bool,
    pub edit_products: bool,
    pub view_products: bool,
    pub view_suppliers: bool,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    pub id: i32,
    pub products: Vec<Option<i32>>,
    pub name: String,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone)]
pub struct Supplier {
    pub id: i32,
    pub products: Vec<Option<i32>>,
    pub name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone)]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub products: Vec<Option<i32>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct ReceivedOrder {
    pub id: i32,
    pub received: Option<NaiveDateTime>,
    pub product_id: i32,
    pub gross_amount: f64,
    pub actually_received: f64,
    pub damaged: f64,
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct PendingOrder {
    pub id: i32,
    pub product_id: i32,
    pub amount: f64,
}
