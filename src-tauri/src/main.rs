#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod client;
mod models;

use anyhow::{Context, Error};
use asciimath::{eval, scope, Scope};
use bigdecimal::{BigDecimal, Zero};
use chrono::{NaiveDate, NaiveDateTime};
use client::Api;
use futures::lock::Mutex;
use models::{Brand, Category, PendingOrder, Product, ReceivedOrder, Supplier};
use ordered_float::NotNan;
use rust_fuzzy_search::fuzzy_search_sorted;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};

extern crate lazy_static;
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::f64::consts::{E, PI};
use std::num::ParseFloatError;
use std::ops::{Add, Div, Mul, Sub};

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref PHONE_NUMBER_REGEX: Regex = Regex::new(r"^(?:(?:\+?1\s*(?:[.-]\s*)?)?(?:\(\s*([2-9]1[02-9]|[2-9][02-8]1|[2-9][02-8][02-9])\s*\)|([2-9]1[02-9]|[2-9][02-8]1|[2-9][02-8][02-9]))\s*(?:[.-]\s*)?)?([2-9]1[02-9]|[2-9][02-9]1|[2-9][02-9]{2})\s*(?:[.-]\s*)?([0-9]{4})(?:\s*(?:#|x\.?|ext\.?|extension)\s*(\d+))?$").unwrap();
}

#[derive(Clone, Deserialize, Serialize, Default)]
struct AppProduct {
    id: i32,
    name: String,
    upc: String,
    description: String,
    buyLevel: Option<f64>,
    costPrice: String,
    sellingPrice: String,
    amount: f64,
    case_size: Option<i32>,
    measureByWeight: bool,
}

#[derive(Clone, Deserialize, Serialize, Default)]
struct AppBrand {
    id: i32,
    name: String,
    products: Vec<i32>,
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
struct AppSupplier {
    id: i32,
    name: String,
    email: String,
    phoneNumber: String,
    products: Vec<i32>,
}

#[derive(Clone, Deserialize, Serialize, Default)]
struct AppPendingOrder {
    id: i32,
    product: i32,
    amount: f64,
}

#[derive(Clone, Deserialize, Serialize, Default)]
struct AppReceivedOrder {
    id: i32,
    product_id: i32,
    gross_amount: f64,
    actually_received: f64,
    damaged: f64,
    received: String,
}

impl AppReceivedOrder {
    fn from_order(order: ReceivedOrder) -> Self {
        AppReceivedOrder {
            id: order.id,
            product_id: order.product_id,
            gross_amount: order.gross_amount,
            actually_received: order.actually_received,
            damaged: order.damaged,
            received: order
                .received
                .unwrap_or_else(|| NaiveDateTime::default())
                .date()
                .format("%m/%d/%Y")
                .to_string(),
        }
    }
    fn to_order(&self) -> Result<ReceivedOrder, Error> {
        Ok(ReceivedOrder {
            id: self.id,
            product_id: self.product_id,
            actually_received: self.actually_received,
            damaged: self.damaged,
            received: Some(
                NaiveDate::parse_from_str(&self.received, "%m/%d/%Y")?
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
            gross_amount: self.gross_amount,
        })
    }
}

impl AppPendingOrder {
    fn from_order(order: PendingOrder) -> Self {
        AppPendingOrder {
            id: order.id,
            product: order.product_id,
            amount: order.amount,
        }
    }
    fn to_order(&self) -> Result<PendingOrder, Error> {
        Ok(PendingOrder {
            id: self.id,
            product_id: self.product,
            amount: self.amount,
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Default)]
struct AppCategory {
    id: i32,
    name: String,
    products: Vec<i32>,
}

impl AppCategory {
    fn from_category(category: Category) -> Self {
        AppCategory {
            id: category.id,
            name: category.name,
            products: category
                .products
                .into_iter()
                .map(|product| product.unwrap())
                .collect(),
        }
    }
    fn to_category(&self) -> Result<Category, Error> {
        Ok(Category {
            id: self.id,
            name: self.name.clone(),
            products: self
                .products
                .clone()
                .into_iter()
                .map(|product| Some(product))
                .collect(),
        })
    }
}

impl AppSupplier {
    fn from_supplier(supplier: Supplier) -> Self {
        AppSupplier {
            id: supplier.id,
            name: supplier.name,
            email: supplier.email.unwrap_or_default(),
            phoneNumber: supplier.phone_number.unwrap_or_default(),
            products: supplier
                .products
                .into_iter()
                .map(|product| product.unwrap())
                .collect(),
        }
    }

    fn to_supplier(&self) -> Result<Supplier, Error> {
        Ok(Supplier {
            id: self.id,
            name: self.name.clone(),
            products: self
                .products
                .clone()
                .into_iter()
                .map(|product| Some(product))
                .collect(),
            phone_number: match self.phoneNumber.as_str() {
                "" => None,
                _ => Some(self.phoneNumber.clone()),
            },
            email: match self.email.as_str() {
                "" => None,
                _ => Some(self.email.clone()),
            },
        })
    }
}

impl AppBrand {
    fn from_brand(brand: Brand) -> Self {
        AppBrand {
            id: brand.id,
            name: brand.name,
            products: brand
                .products
                .into_iter()
                .map(|product| product.unwrap())
                .collect(),
        }
    }
    fn to_brand(&self) -> Result<Brand, Error> {
        Ok(Brand {
            id: self.id,
            name: self.name.clone(),
            products: self
                .products
                .clone()
                .into_iter()
                .map(|product| Some(product))
                .collect(),
        })
    }
}

impl AppProduct {
    fn from_product(product: Product) -> Self {
        AppProduct {
            id: product.id,
            name: product.name,
            upc: product.upc,
            description: product.description,
            buyLevel: product.buy_level,
            costPrice: product.cost_price_per_unit.to_string(),
            sellingPrice: product.selling_price_per_unit.to_string(),
            amount: product.amount,
            case_size: product.case_size,
            measureByWeight: product.measure_by_weight,
        }
    }
    fn to_product(&self) -> Result<Product, Error> {
        Ok(Product {
            id: self.id,
            upc: self.upc.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            buy_level: self.buyLevel,
            amount: self.amount,
            case_size: self.case_size,
            sale_end: None,
            sale_price: None,
            measure_by_weight: self.measureByWeight,
            cost_price_per_unit: BigDecimal::from_str(&self.costPrice)?,
            selling_price_per_unit: BigDecimal::from_str(&self.sellingPrice)?,
        })
    }
}

fn err_to_string(err: anyhow::Error) -> String {
    err.to_string()
}

#[derive(Deserialize, Serialize, Debug)]
struct ProductName {
    name: String,
    upc: String,
    id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct BrandName {
    name: String,
    id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct SupplierName {
    name: String,
    id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct CategoryName {
    name: String,
    id: i32,
}

#[tauri::command]
async fn category_names(state: tauri::State<'_, AppState>) -> Result<Vec<CategoryName>, String> {
    Ok(state
        .0
        .lock()
        .await
        .category_names()
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|(name, id)| CategoryName { name, id })
        .collect())
}

#[tauri::command]
async fn sort_categories(
    state: tauri::State<'_, AppState>,
    category_names: Vec<CategoryName>,
    search: &str,
) -> Result<Vec<CategoryName>, String> {
    let mut category_names = category_names.into_iter().collect::<Vec<_>>();
    category_names.sort_by_cached_key(|category| {
        NotNan::new(rust_fuzzy_search::fuzzy_compare(search, &category.name)).unwrap()
        // compiler won't let me sort f64 due to possible NaN
    });
    category_names.reverse();
    Ok(category_names)
}

#[tauri::command]
async fn supplier_names(state: tauri::State<'_, AppState>) -> Result<Vec<SupplierName>, String> {
    Ok(state
        .0
        .lock()
        .await
        .supplier_names()
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|(name, id)| SupplierName { name, id })
        .collect())
}

#[tauri::command]
async fn sort_brands(
    state: tauri::State<'_, AppState>,
    brand_names: Vec<BrandName>,
    search: &str,
) -> Result<Vec<BrandName>, String> {
    let mut brand_names = brand_names.into_iter().collect::<Vec<_>>();
    brand_names.sort_by_cached_key(|brand| {
        NotNan::new(rust_fuzzy_search::fuzzy_compare(search, &brand.name)).unwrap()
        // compiler won't let me sort f64 due to possible NaN
    });
    brand_names.reverse();
    Ok(brand_names)
}

#[tauri::command]
async fn sort_suppliers(
    state: tauri::State<'_, AppState>,
    supplier_names: Vec<SupplierName>,
    search: &str,
) -> Result<Vec<SupplierName>, String> {
    let mut supplier_names = supplier_names.into_iter().collect::<Vec<_>>();
    supplier_names.sort_by_cached_key(|supplier| {
        NotNan::new(rust_fuzzy_search::fuzzy_compare(search, &supplier.name)).unwrap()
        // compiler won't let me sort f64 due to possible NaN
    });
    supplier_names.reverse();
    Ok(supplier_names)
}

#[tauri::command]
async fn brand_names(state: tauri::State<'_, AppState>) -> Result<Vec<BrandName>, String> {
    Ok(state
        .0
        .lock()
        .await
        .brand_names()
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|(name, id)| BrandName { name, id })
        .collect())
}

#[tauri::command]
async fn product_names(state: tauri::State<'_, AppState>) -> Result<Vec<ProductName>, String> {
    Ok(state
        .0
        .lock()
        .await
        .product_names()
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|(name, upc, id)| ProductName { name, upc, id })
        .collect())
}

#[tauri::command]
async fn sort_products(
    state: tauri::State<'_, AppState>,
    product_names: Vec<ProductName>,
    search: &str,
) -> Result<Vec<ProductName>, String> {
    if search.parse::<f64>().is_ok() {
        let mut product_names = product_names.into_iter().collect::<Vec<_>>();
        product_names.sort_by_cached_key(|product| {
            NotNan::new(rust_fuzzy_search::fuzzy_compare(search, &product.upc)).unwrap()
            // compiler won't let me sort f64 due to possible NaN
        });
        product_names.reverse();
        Ok(product_names)
    } else {
        let mut product_names = product_names.into_iter().collect::<Vec<_>>();
        product_names.sort_by_cached_key(|product| {
            NotNan::new(rust_fuzzy_search::fuzzy_compare(search, &product.name)).unwrap()
            // compiler won't let me sort f64 due to possible NaN
        });
        product_names.reverse();
        Ok(product_names)
    }
}

#[tauri::command]
async fn calc(state: tauri::State<'_, AppState>, input_str: &str) -> Result<f64, String> {
    return Ok(eval(input_str, &scope! {})
        .context("")
        .map_err(err_to_string)?);
}

struct AppState(Arc<Mutex<Api>>);

#[tauri::command]
async fn log_in(
    state: tauri::State<'_, AppState>,
    username: &str,
    password: &str,
) -> Result<(), String> {
    // I don't want to set the mutex contents on an error here
    let api = Api::new(username, password).await.map_err(err_to_string)?;

    // Introduce an extra scope to force the mutex guard to release when the scope
    // closes.
    {
        let mut client = state.0.lock().await;
        *client = api;
        client.permissions().await.map_err(err_to_string)?
    };

    Ok(())
}

#[tauri::command]
async fn get_products(
    state: tauri::State<'_, AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AppProduct>, String> {
    Ok(state
        .0
        .lock()
        .await
        .get_products(limit, offset)
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(AppProduct::from_product)
        .collect())
}

#[tauri::command]
async fn remove_received_order(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .remove_received_order(id)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn remove_category(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .remove_category(id)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn remove_product(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .remove_product(id)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn remove_pending_order(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .remove_pending_order(id)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn remove_brand(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .remove_brand(id)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn remove_supplier(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    state
        .0
        .lock()
        .await
        .remove_supplier(id)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn get_brands(
    state: tauri::State<'_, AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AppBrand>, String> {
    Ok(state
        .0
        .lock()
        .await
        .get_brands(limit, offset)
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(AppBrand::from_brand)
        .collect())
}

#[tauri::command]
async fn get_categories(
    state: tauri::State<'_, AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Category>, String> {
    state
        .0
        .lock()
        .await
        .get_categories(limit, offset)
        .await
        .map_err(err_to_string)
}

#[tauri::command]
async fn get_received_orders(
    state: tauri::State<'_, AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AppReceivedOrder>, String> {
    Ok(state
        .0
        .lock()
        .await
        .get_received_orders(limit, offset)
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|order| AppReceivedOrder::from_order(order))
        .collect())
}

#[tauri::command]
async fn get_pending_orders(
    state: tauri::State<'_, AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AppPendingOrder>, String> {
    Ok(state
        .0
        .lock()
        .await
        .get_pending_orders(limit, offset)
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|order| AppPendingOrder::from_order(order))
        .collect())
}

#[tauri::command]
async fn mark_order_received(
    state: tauri::State<'_, AppState>,
    order: AppPendingOrder,
    date: String,
    actually_received: f64,
    damaged: f64,
) -> Result<AppReceivedOrder, String> {
    println!("{}", date);
    let mut received = AppReceivedOrder {
        id: order.id,
        product_id: order.product,
        received: date.clone(),
        damaged,
        gross_amount: order.amount,
        actually_received,
    };
    println!("{}", date);
    let date = NaiveDate::parse_from_str(&date, "%m/%d/%Y")
        .context("Can't convert string to datetime")
        .map_err(err_to_string)?
        .and_hms_opt(0, 0, 0)
        .context("Can't convert date to datetime")
        .map_err(err_to_string)?;
    let id = state
        .0
        .lock()
        .await
        .mark_as_received(order.id, date, actually_received, damaged)
        .await
        .map_err(err_to_string)?;
    received.id = id;
    Ok(received)
}

#[tauri::command]
async fn get_suppliers(
    state: tauri::State<'_, AppState>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Supplier>, String> {
    state
        .0
        .lock()
        .await
        .get_suppliers(limit, offset)
        .await
        .map_err(err_to_string)
}

#[tauri::command]
async fn save_brand(state: tauri::State<'_, AppState>, brand: AppBrand) -> Result<(), String> {
    let brand = brand.to_brand().map_err(err_to_string)?;
    state
        .0
        .lock()
        .await
        .update_brand(&brand)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn validate_phone_number(
    state: tauri::State<'_, AppState>,
    input_str: &str,
) -> Result<bool, String> {
    Ok(PHONE_NUMBER_REGEX.is_match(input_str) || input_str == "")
}

#[tauri::command]
async fn validate_email(
    state: tauri::State<'_, AppState>,
    input_str: &str,
) -> Result<bool, String> {
    Ok(EMAIL_REGEX.is_match(input_str) || input_str == "")
}
// comment
#[tauri::command]
async fn save_supplier(
    state: tauri::State<'_, AppState>,
    supplier: AppSupplier,
) -> Result<(), String> {
    if (&supplier.phoneNumber == ""
        || (!PHONE_NUMBER_REGEX.is_match(&supplier.phoneNumber))
        || (&supplier.email == "" || !EMAIL_REGEX.is_match(&supplier.email)))
    {
        return Err(String::from("Not a valid input"));
    }
    let supplier = supplier.to_supplier().map_err(err_to_string)?;
    println!("{:?}", supplier);
    state
        .0
        .lock()
        .await
        .update_supplier(&supplier)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn save_category(
    state: tauri::State<'_, AppState>,
    category: AppCategory,
) -> Result<(), String> {
    let category = category.to_category().map_err(err_to_string)?;
    state
        .0
        .lock()
        .await
        .update_category(&category)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn save_received_order(
    state: tauri::State<'_, AppState>,
    order: AppReceivedOrder,
) -> Result<(), String> {
    println!("{}", order.received);
    state
        .0
        .lock()
        .await
        .update_received_order(&order.to_order().map_err(err_to_string)?)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn save_product(
    state: tauri::State<'_, AppState>,
    product: AppProduct,
) -> Result<(), String> {
    let product = product.to_product().map_err(err_to_string)?;
    state
        .0
        .lock()
        .await
        .update_product(&product)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn save_pending_order(
    state: tauri::State<'_, AppState>,
    order: AppPendingOrder,
) -> Result<(), String> {
    let order = order.to_order().map_err(err_to_string)?;
    state
        .0
        .lock()
        .await
        .update_pending_order(&order)
        .await
        .map_err(err_to_string)?;
    Ok(())
}

#[tauri::command]
async fn new_brand(state: tauri::State<'_, AppState>) -> Result<AppBrand, String> {
    let id = state
        .0
        .lock()
        .await
        .new_brand("")
        .await
        .map_err(err_to_string)?;
    Ok({
        let mut brand = AppBrand::default();
        brand.id = id;
        brand
    })
}

#[tauri::command]
async fn new_supplier(state: tauri::State<'_, AppState>) -> Result<AppSupplier, String> {
    let id = state
        .0
        .lock()
        .await
        .new_supplier("", "", "")
        .await
        .map_err(err_to_string)?;

    Ok({
        let mut supplier = AppSupplier::default();
        supplier.id = id;
        supplier
    })
}

#[tauri::command]
async fn new_category(state: tauri::State<'_, AppState>) -> Result<AppCategory, String> {
    let id = state
        .0
        .lock()
        .await
        .new_category("")
        .await
        .map_err(err_to_string)?;
    Ok({
        let mut category = AppCategory::default();
        category.id = id;
        category
    })
}

#[tauri::command]
async fn new_pending_order(
    state: tauri::State<'_, AppState>,
    product_id: i32,
) -> Result<AppPendingOrder, String> {
    let id = state
        .0
        .lock()
        .await
        .new_pending_order(0.0, product_id)
        .await
        .map_err(err_to_string)?;
    Ok({
        let mut order = AppPendingOrder::default();
        order.id = id;
        order.product = product_id;
        order
    })
}

#[tauri::command]
async fn product_brand(state: tauri::State<'_, AppState>, id: i32) -> Result<AppBrand, String> {
    Ok(AppBrand::from_brand(
        state
            .0
            .lock()
            .await
            .get_product_brand(id)
            .await
            .map_err(err_to_string)?
            .context("No brand")
            .map_err(err_to_string)?,
    ))
}

#[tauri::command]
async fn product_suppliers(
    state: tauri::State<'_, AppState>,
    id: i32,
) -> Result<Vec<AppSupplier>, String> {
    Ok(state
        .0
        .lock()
        .await
        .get_product_suppliers(id)
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|supplier| AppSupplier::from_supplier(supplier))
        .collect())
}

#[tauri::command]
async fn product_categories(
    state: tauri::State<'_, AppState>,
    id: i32,
) -> Result<Vec<AppCategory>, String> {
    Ok(state
        .0
        .lock()
        .await
        .get_product_categories(id)
        .await
        .map_err(err_to_string)?
        .into_iter()
        .map(|category| AppCategory::from_category(category))
        .collect())
}

#[tauri::command]
async fn new_product(state: tauri::State<'_, AppState>) -> Result<AppProduct, String> {
    let id = state
        .0
        .lock()
        .await
        .new_product(
            "",
            "",
            "",
            false,
            BigDecimal::zero(),
            BigDecimal::zero(),
            0.0,
            Vec::new(),
            Vec::new(),
            None,
        )
        .await
        .map_err(err_to_string)?;
    Ok({
        let product = AppProduct {
            id,
            name: String::from(""),
            upc: String::from(""),
            description: String::from(""),
            buyLevel: Some(0.0),
            costPrice: String::from(""),
            sellingPrice: String::from(""),
            amount: 0.0,
            case_size: Some(0),
            measureByWeight: false,
        };
        product
    })
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(AppState(Arc::new(Mutex::new(
            Api::new("", "").await.unwrap(),
        ))))
        .invoke_handler(tauri::generate_handler![
            log_in,
            get_products,
            get_brands,
            get_categories,
            get_pending_orders,
            get_suppliers,
            remove_product,
            remove_pending_order,
            new_product,
            save_received_order,
            new_pending_order,
            save_product,
            save_brand,
            new_brand,
            mark_order_received,
            get_received_orders,
            remove_received_order,
            new_supplier,
            save_supplier,
            save_pending_order,
            remove_brand,
            remove_supplier,
            save_category,
            sort_products,
            remove_category,
            new_category,
            calc,
            validate_email,
            validate_phone_number,
            product_names,
            category_names,
            supplier_names,
            product_categories,
            product_brand,
            product_suppliers,
            sort_brands,
            sort_suppliers,
            sort_categories,
            brand_names,
        ])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
