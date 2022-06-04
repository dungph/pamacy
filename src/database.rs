use std::vec;

use anyhow::{Ok, Result};
use async_std::sync::Mutex;
use chrono::{Duration, Utc};
use once_cell::sync::Lazy;
use sqlx::PgPool;

static DB: Lazy<PgPool> = Lazy::new(|| {
    PgPool::connect_lazy(
        std::env::var("DATABASE_URL")
            .expect("set DATABASE_URL")
            .as_str(),
    )
    .expect("DB connect lazy failed")
});

#[derive(Clone, Debug)]
pub(crate) struct DrugInfo {
    pub medicine_id: i32,
    pub medicine_expire_date: chrono::DateTime<Utc>,
    pub medicine_price: i32,
    pub medicine_code: String,
    pub medicine_name: String,
    pub medicine_content: String,
    pub medicine_element: String,
    pub medicine_group: String,
    pub supplier: String,
    pub quantity: i32,
}
static DRUG_DB: Lazy<Mutex<Vec<DrugInfo>>> = Lazy::new(|| {
    let sample1 = DrugInfo {
        medicine_id: 1,
        medicine_expire_date: Utc::now() + Duration::days(300),
        medicine_price: 100000,
        medicine_code: String::from("MABF"),
        medicine_name: String::from("Thuoc MABF"),
        medicine_content: String::from("M A B F"),
        medicine_element: String::from("M A B F"),
        medicine_group: String::from("M A B F"),
        supplier: String::from("Company A"),
        quantity: 100,
    };
    let sample2 = DrugInfo {
        medicine_id: 2,
        medicine_expire_date: Utc::now() + Duration::days(300),
        medicine_price: 80000,
        medicine_code: String::from("GFEF"),
        medicine_name: String::from("Thuoc GFEF"),
        medicine_content: String::from("M A B F"),
        medicine_element: String::from("M A B F"),
        medicine_group: String::from("M A B F"),
        supplier: String::from("Company B"),
        quantity: 80,
    };
    let sample3 = DrugInfo {
        medicine_id: 3,
        medicine_expire_date: Utc::now() + Duration::days(300),
        medicine_price: 100000,
        medicine_code: String::from("TRE"),
        medicine_name: String::from("Thuoc TRE"),
        medicine_content: String::from("M A B F"),
        medicine_element: String::from("M A B F"),
        medicine_group: String::from("M A B F"),
        supplier: String::from("Company B"),
        quantity: 100,
    };
    Mutex::new(vec![sample3, sample2, sample1])
});

pub(super) async fn find_drug_match_any(
    medicine_code: Option<String>,
    medicine_name: Option<String>,
    medicine_content: Option<String>,
) -> Result<Vec<DrugInfo>> {
    Ok(list_drug()
        .await?
        .iter()
        .filter(|drug| {
            medicine_code
                .as_ref()
                .map(|c| c.as_str() == drug.medicine_code.as_str())
                .unwrap_or(false)
                || medicine_name
                    .as_ref()
                    .map(|c| c.as_str() == drug.medicine_name.as_str())
                    .unwrap_or(false)
                || medicine_content
                    .as_ref()
                    .map(|c| c.as_str() == drug.medicine_content.as_str())
                    .unwrap_or(false)
        })
        .cloned()
        .collect())
}

pub(crate) async fn list_drug() -> anyhow::Result<Vec<DrugInfo>> {
    Ok(DRUG_DB.lock().await.clone())
}

pub(crate) async fn add_drug(drug: DrugInfo) -> anyhow::Result<()> {
    DRUG_DB.lock().await.push(drug);
    Ok(())
}
