use std::{collections::HashSet, vec};

use anyhow::{Ok, Result};
use async_std::sync::Mutex;
use chrono::{Duration, Utc};
use once_cell::sync::Lazy;
use serde::Serialize;
use sqlx::PgPool;

static DB: Lazy<PgPool> = Lazy::new(|| {
    PgPool::connect_lazy(
        std::env::var("DATABASE_URL")
            .expect("set DATABASE_URL")
            .as_str(),
    )
    .expect("DB connect lazy failed")
});

#[derive(Clone, Debug, Serialize, Default)]
pub(crate) struct DrugInfo {
    pub medicine_id: i32,
    pub medicine_import_date: String,
    pub medicine_expire_date: String,
    pub medicine_price: i32,
    pub medicine_code: String,
    pub medicine_name: String,
    pub medicine_type: String,
    pub medicine_content: String,
    pub medicine_element: String,
    pub medicine_group: String,
    pub supplier: String,
    pub medicine_quantity: i32,
    pub medicine_location: String,
}

static DRUG_DB: Lazy<Mutex<Vec<DrugInfo>>> = Lazy::new(|| {
    let sample1 = DrugInfo {
        medicine_id: 1,
        medicine_expire_date: (Utc::now() + Duration::days(300)).to_rfc2822(),
        medicine_import_date: Utc::now().to_rfc2822(),
        medicine_price: 100000,
        medicine_code: String::from("MABF"),
        medicine_name: String::from("Thuoc MABF"),
        medicine_content: String::from("M A B F"),
        medicine_element: String::from("M A B F"),
        medicine_group: String::from("M A B F"),
        supplier: String::from("Company A"),
        medicine_quantity: 100,
        medicine_type: String::from("Type 2"),
        medicine_location: String::from("Location A"),
    };
    let sample2 = DrugInfo {
        medicine_id: 2,
        medicine_expire_date: (Utc::now() + Duration::days(300)).to_rfc2822(),
        medicine_import_date: Utc::now().to_rfc2822(),
        medicine_price: 80000,
        medicine_code: String::from("GFEF"),
        medicine_type: String::from("Type 1"),
        medicine_name: String::from("Thuoc GFEF"),
        medicine_content: String::from("M A B F"),
        medicine_element: String::from("M A B F"),
        medicine_group: String::from("M A B F"),
        supplier: String::from("Company B"),
        medicine_quantity: 80,
        medicine_location: String::from("Location A"),
    };
    let sample3 = DrugInfo {
        medicine_id: 3,
        medicine_expire_date: (Utc::now() + Duration::days(300)).to_rfc2822(),
        medicine_import_date: Utc::now().to_rfc2822(),
        medicine_price: 100000,
        medicine_code: String::from("TRE"),
        medicine_type: String::from("Type 1"),
        medicine_name: String::from("Thuoc TRE"),
        medicine_content: String::from("M A B F"),
        medicine_element: String::from("M A B F"),
        medicine_group: String::from("M A B F"),
        supplier: String::from("Company B"),
        medicine_quantity: 100,
        medicine_location: String::from("Location A"),
    };
    Mutex::new(vec![sample1, sample2, sample3])
});

pub(super) async fn find_drug_match_any(
    medicine_id: Option<String>,
    medicine_name: Option<String>,
    medicine_type: Option<String>,
) -> Result<Vec<DrugInfo>> {
    Ok(list_drug()
        .await?
        .iter()
        .filter(|drug| {
            medicine_id
                .as_ref()
                .map(|c| {
                    drug.medicine_code
                        .to_lowercase()
                        .contains(&c.to_lowercase())
                })
                .unwrap_or(false)
                || medicine_name
                    .as_ref()
                    .map(|c| {
                        drug.medicine_name
                            .to_lowercase()
                            .contains(&c.to_lowercase())
                    })
                    .unwrap_or(true)
        })
        .filter(|drug| {
            medicine_type
                .as_ref()
                .map(|c| {
                    drug.medicine_type
                        .to_lowercase()
                        .contains(&c.to_lowercase())
                })
                .unwrap_or(true)
        })
        .cloned()
        .collect())
}

pub(crate) async fn list_drug() -> anyhow::Result<Vec<DrugInfo>> {
    Ok(DRUG_DB.lock().await.clone())
}
pub(crate) async fn list_drug_type() -> anyhow::Result<HashSet<String>> {
    Ok(DRUG_DB
        .lock()
        .await
        .iter()
        .map(|drug| drug.medicine_type.clone())
        .collect())
}
pub(crate) async fn add_drug(drug: DrugInfo) -> anyhow::Result<()> {
    DRUG_DB.lock().await.push(drug);
    Ok(())
}
pub(crate) async fn next_drug_id() -> anyhow::Result<i32> {
    Ok(DRUG_DB
        .lock()
        .await
        .last()
        .map(|drug| drug.medicine_id + 1)
        .unwrap_or(1))
}
