use anyhow::Context;
use bigdecimal::BigDecimal;
use futures::executor::block_on;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Url,
};

use chrono::NaiveDateTime;

use crate::models::*;
use anyhow::anyhow;

const BASE_URL: &str = "https://d3v3ai4t8a3aev.cloudfront.net/";

pub struct Api {
    client: Client,
    headers: HeaderMap,
}

impl Api {
    pub async fn new(user_name: &str, password: &str) -> Result<Self, anyhow::Error> {
        let mut headers = HeaderMap::new();
        headers.insert("username", HeaderValue::from_str(user_name)?);
        headers.insert("password", HeaderValue::from_str(password)?);
        let client = Client::new();

        client
            .get(Url::parse(
                Url::parse(BASE_URL)?
                    .join("/initialize/")?
                    .join(&(user_name.to_string() + "/"))?
                    .join(&(password.to_string() + "/"))?
                    .as_str(),
            )?)
            .send()
            .await
            .with_context(|| "can't create")?;

        Ok(Self { client, headers })
    }

    pub async fn permissions(&self) -> Result<Permission, anyhow::Error> {
        let thing = self
            .client
            .get(Url::parse(BASE_URL)?.join("/permissions")?.as_str())
            .headers(self.headers.clone());

        Ok(self
            .client
            .get(Url::parse(BASE_URL)?.join("/permissions")?.as_str())
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<Permission>()
            .await?)
    }

    pub async fn sign_up(&self, user_name: &str, password: &str) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(Url::parse_with_params(
                Url::parse(BASE_URL)?.join("/signup")?.as_str(),
                &[
                    ("username", user_name.to_string()),
                    ("password", password.to_string()),
                ],
            )?)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<Option<()>>()
            .await?;
        if response.is_none() {
            return Err(anyhow!("Can't sign user up"));
        } else {
            Ok(())
        }
    }

    pub async fn update_user(&self, user: &User) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/update_user")?
                        .join(
                            &serde_json::to_string(user)
                                .context("Failed to serialize user_info")?,
                        )?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_product(&self, product: &Product) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/update_product/")?
                        .join(
                            &serde_json::to_string(product)
                                .context("Failed to serialize user_info")?,
                        )?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_supplier(&self, supplier: &Supplier) -> Result<(), anyhow::Error> {
        println!(
            "{}",
            serde_json::to_string(supplier).context("Failed to serialize supplier info")?
        );
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/update_supplier")?.as_str(),
                    &[(
                        "supplier_info",
                        serde_json::to_string(supplier)
                            .context("Failed to serialize supplier info")?,
                    )],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_brand(&self, brand: &Brand) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/update_brand")?.as_str(),
                    &[(
                        "brand_info",
                        serde_json::to_string(brand).context("Failed to serialize user_info")?,
                    )],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_category(&self, category: &Category) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/update_category")?.as_str(),
                    &[(
                        "category_info",
                        serde_json::to_string(category).context("Failed to serialize user_info")?,
                    )],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_pending_order(&self, order: &PendingOrder) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?
                        .join("/update_pending_order")?
                        .as_str(),
                    &[(
                        "order_info",
                        serde_json::to_string(order).context("Failed to serialize user_info")?,
                    )],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_received_order(&self, order: &ReceivedOrder) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?
                        .join("/update_received_order")?
                        .as_str(),
                    &[(
                        "order_info",
                        serde_json::to_string(order).context("Failed to serialize user_info")?,
                    )],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn new_brand(&self, name: &str) -> Result<i32, anyhow::Error> {
        self.client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/new_brand")?.as_str(),
                    &[("name", name.to_string())],
                )
                .context("Couldn't create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<i32>()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn new_pending_order(
        &self,
        amount: f64,
        product_id: i32,
    ) -> Result<i32, anyhow::Error> {
        self.client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/new_pending_order")?.as_str(),
                    &[
                        ("amount", amount.to_string()),
                        ("product_id", product_id.to_string()),
                    ],
                )
                .context("Couldn't create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<i32>()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn product_names(&self) -> Result<Vec<(String, String, i32)>, anyhow::Error> {
        self.client
            .get(Url::parse(BASE_URL)?.join("/products/names")?.as_str())
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn category_names(&self) -> Result<Vec<(String, i32)>, anyhow::Error> {
        self.client
            .get(Url::parse(BASE_URL)?.join("/categories/names")?.as_str())
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn supplier_names(&self) -> Result<Vec<(String, i32)>, anyhow::Error> {
        self.client
            .get(Url::parse(BASE_URL)?.join("/suppliers/names")?.as_str())
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn brand_names(&self) -> Result<Vec<(String, i32)>, anyhow::Error> {
        self.client
            .get(Url::parse(BASE_URL)?.join("/brands/names")?.as_str())
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn new_category(&self, name: &str) -> Result<i32, anyhow::Error> {
        self.client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/new_category")?.as_str(),
                    &[("name", name.to_string())],
                )
                .context("Couldn't create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<i32>()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn new_supplier(
        &self,
        name: &str,
        phone_number: &str,
        email: &str,
    ) -> Result<i32, anyhow::Error> {
        self.client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/new_supplier")?.as_str(),
                    &[
                        ("name", name.to_string()),
                        ("phone_number", phone_number.to_string()),
                        ("email", email.to_string()),
                    ],
                )
                .context("Couldn't create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<i32>()
            .await
            .context("Couldn't convert result to json")
    }

    pub async fn new_product(
        &self,
        upc: &str,
        name: &str,
        description: &str,
        measure_by_weight: bool,
        cost_price_per_unit: BigDecimal,
        selling_price_per_unit: BigDecimal,
        buy_level: f64,
        categories: Vec<i32>,
        suppliers: Vec<i32>,
        brand: Option<i32>,
    ) -> Result<i32, anyhow::Error> {
        let response = if brand.is_some() {
            self.client
                .get(
                    Url::parse_with_params(
                        Url::parse(BASE_URL)?.join("/new_product")?.as_str(),
                        &[
                            ("upc", upc.to_string()),
                            ("name", name.to_string()),
                            ("description", description.to_string()),
                            ("measure_by_weight", measure_by_weight.to_string()),
                            ("cost_price_per_unit", cost_price_per_unit.to_string()),
                            ("selling_price_per_unit", selling_price_per_unit.to_string()),
                            ("categories", serde_json::to_string(&categories)?),
                            ("suppliers", serde_json::to_string(&suppliers)?),
                            ("brand", brand.unwrap().to_string()),
                            ("buy_level", buy_level.to_string()),
                        ],
                    )
                    .context("Failed to create url string")?,
                )
                .headers(self.headers.clone())
                .send()
                .await?
                .json::<i32>()
                .await?
        } else {
            self.client
                .get(
                    Url::parse_with_params(
                        Url::parse(BASE_URL)?.join("/new_product")?.as_str(),
                        &[
                            ("upc", upc.to_string()),
                            ("name", name.to_string()),
                            ("description", description.to_string()),
                            ("measure_by_weight", measure_by_weight.to_string()),
                            ("cost_price_per_unit", cost_price_per_unit.to_string()),
                            ("selling_price_per_unit", selling_price_per_unit.to_string()),
                            ("categories", serde_json::to_string(&categories)?),
                            ("suppliers", serde_json::to_string(&suppliers)?),
                            ("buy_level", buy_level.to_string()),
                        ],
                    )
                    .context("Failed to create url string")?,
                )
                .headers(self.headers.clone())
                .send()
                .await?
                .json::<i32>()
                .await?
        };
        Ok(response)
    }

    pub async fn remove_product(&self, id: i32) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/remove_product/")?
                        .join(&id.to_string())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn remove_category(&self, id: i32) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/remove_category/")?
                        .join(&id.to_string())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn remove_brand(&self, id: i32) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/remove_brand/")?
                        .join(&id.to_string())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn remove_supplier(&self, id: i32) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/remove_supplier/")?
                        .join(&id.to_string())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn remove_pending_order(&self, id: i32) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/remove_pending_order/")?
                        .join(&id.to_string())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn remove_received_order(&self, id: i32) -> Result<(), anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/remove_received_order/")?
                        .join(&id.to_string())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_products(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Product>, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/products")?.as_str(),
                    &[("limit", limit.to_string()), ("offset", offset.to_string())],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_pending_orders(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PendingOrder>, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/pending_orders")?.as_str(),
                    &[("limit", limit.to_string()), ("offset", offset.to_string())],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_received_orders(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ReceivedOrder>, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/received_orders")?.as_str(),
                    &[("limit", limit.to_string()), ("offset", offset.to_string())],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_brands(&self, limit: i64, offset: i64) -> Result<Vec<Brand>, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/brands")?.as_str(),
                    &[("limit", limit.to_string()), ("offset", offset.to_string())],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_categories(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Category>, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/categories")?.as_str(),
                    &[("limit", limit.to_string()), ("offset", offset.to_string())],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_suppliers(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Supplier>, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?.join("/suppliers")?.as_str(),
                    &[("limit", limit.to_string()), ("offset", offset.to_string())],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_category(&self, id: i32) -> Result<Category, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/category/")?
                        .join(id.to_string().as_str())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_supplier(&self, id: i32) -> Result<Supplier, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/supplier/")?
                        .join(id.to_string().as_str())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_product(&self, id: i32) -> Result<Product, anyhow::Error> {
        let response = self
            .client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/product/")?
                        .join(id.to_string().as_str())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json();
        Ok(response.await?)
    }

    pub async fn get_product_brand(&self, product: i32) -> Result<Option<Brand>, anyhow::Error> {
        self.client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/product_brand/")?
                        .join(product.to_string().as_str())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await
            .context("Can't send request")?
            .json()
            .await
            .context("Can't change to json")
    }

    pub async fn get_product_suppliers(
        &self,
        product: i32,
    ) -> Result<Vec<Supplier>, anyhow::Error> {
        self.client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/product_suppliers/")?
                        .join(product.to_string().as_str())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await
            .context("Can't send request")?
            .json()
            .await
            .context("Can't change to json")
    }

    pub async fn mark_as_received(
        &self,
        id: i32,
        date: NaiveDateTime,
        actually_received: f64,
        damaged: f64,
    ) -> Result<i32, anyhow::Error> {
        self.client
            .get(
                Url::parse_with_params(
                    Url::parse(BASE_URL)?
                        .join("/mark_order_as_received")?
                        .as_str(),
                    &[
                        ("order_id", id.to_string()),
                        ("date", date.timestamp().to_string()),
                        ("actually_received", actually_received.to_string()),
                        ("damaged", damaged.to_string()),
                    ],
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await
            .context("Couldn't send request.")
    }

    pub async fn get_product_categories(
        &self,
        product: i32,
    ) -> Result<Vec<Category>, anyhow::Error> {
        self.client
            .get(
                Url::parse(
                    Url::parse(BASE_URL)?
                        .join("/product_categories/")?
                        .join(product.to_string().as_str())?
                        .as_str(),
                )
                .context("Failed to create url string")?,
            )
            .headers(self.headers.clone())
            .send()
            .await
            .context("Can't send request")?
            .json()
            .await
            .context("Can't change to json")
    }
}
