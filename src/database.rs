use std::{collections::HashSet, vec};

use anyhow::Result;
use async_std::sync::Mutex;
use chrono::{DateTime, Duration, Utc};
use once_cell::sync::Lazy;
use serde::Serialize;
use sqlx::{query, query_as, PgPool};

static DB: Lazy<PgPool> = Lazy::new(|| {
    PgPool::connect_lazy(
        std::env::var("DATABASE_URL")
            .expect("set DATABASE_URL")
            .as_str(),
    )
    .expect("DB connect lazy failed")
});

#[derive(Serialize, Debug)]
pub(crate) struct ManageMedicineTemplate {
    medicine_id: i32,
    medicine_name: String,
    medicine_type: String,
    medicine_price: i32,
    medicine_quantity: i32,
    medicine_location: String,
}

pub(crate) async fn find_drug(
    name: String,
    drug_type: String,
) -> Result<Vec<ManageMedicineTemplate>> {
    Ok(query_as!(
        ManageMedicineTemplate,
        r#"select 
                medicine.medicine_id as "medicine_id!",
                medicine_name as "medicine_name!",
                medicine_type as "medicine_type!",
                medicine_price as "medicine_price!",
                medicine_quantity as "medicine_quantity!",
                location_name as "medicine_location!" 
            from medicine
            join quantity on quantity.medicine_id = medicine.medicine_id
            join location on location.location_id = medicine_location_id
            where (medicine_name ~* $1 and medicine_type ~* $2)
            "#,
        name,
        drug_type
    )
    .fetch_all(&*DB)
    .await?)
}

pub(crate) async fn add_drug(
    medicine_name: String,
    medicine_type: String,
    medicine_location: String,
    medicine_price: i32,
    medicine_quantity: i32,
    medicine_import_date: DateTime<Utc>,
    medicine_expire_date: DateTime<Utc>,
) -> Result<()> {
    let location_id: i32 = query!(
        r#"insert into location(location_name)
            values ($1)
            on conflict do nothing
            returning location_id as id;
            "#,
        medicine_location
    )
    .fetch_one(&*DB)
    .await
    .map_err(|ee| anyhow::anyhow!(ee))?
    .id;

    let medicine_id = query!(
        r#"insert into medicine(
                    medicine_name,
                    medicine_type,
                    medicine_location_id,
                    medicine_price,
                    medicine_import_date,
                    medicine_expire_date
                )
                values($1, $2, $3, $4, $5, $6)
                returning medicine_id;
                "#,
        medicine_name,
        medicine_type,
        location_id,
        medicine_price,
        medicine_import_date,
        medicine_expire_date
    )
    .fetch_one(&*DB)
    .await?
    .medicine_id;
    query!(
        r#"insert into quantity(medicine_id, medicine_quantity)
                values ($1, $2)
                "#,
        medicine_id,
        medicine_quantity
    )
    .execute(&*DB)
    .await?;
    Ok(())
}
pub(crate) async fn list_drug_type() -> anyhow::Result<Vec<String>> {
    Ok(query!(
        r#"select medicine_type as "medicine_type!"
              from medicine
              group by medicine_type
              "#,
    )
    .fetch_all(&*DB)
    .await?
    .into_iter()
    .map(|obj| obj.medicine_type)
    .collect())
}
