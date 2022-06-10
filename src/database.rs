use std::{collections::HashSet, vec};

use anyhow::Result;
use async_std::sync::Mutex;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
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
pub(crate) async fn migrate() -> Result<()> {
    sqlx::migrate!().run(&*DB).await?;
    query!(
        r#"
        insert into staff(staff_id, staff_name, staff_username, staff_password)
        values (1, 'Administrator', 'admin', 'admin')
        on conflict (staff_username) do nothing;
        "#
    )
    .execute(&*DB)
    .await?;
    Ok(())
}
#[derive(Serialize, Debug)]
pub(crate) struct ManageMedicineTemplate {
    medicine_id: i32,
    medicine_name: String,
    medicine_type: String,
    medicine_price: i32,
    medicine_quantity: i32,
    medicine_location: String,
    medicine_prescripted: bool,
}

pub(crate) async fn find_drug(
    name: String,
    drug_type: String,
) -> Result<Vec<ManageMedicineTemplate>> {
    Ok(query_as!(
        ManageMedicineTemplate,
        r#"select 
                medicine_id,
                medicine_name,
                medicine_type,
                medicine_price,
                medicine_quantity,
                medicine_location,
                medicine_prescripted
            from medicine
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
    medicine_prescripted: bool,
) -> Result<()> {
    query!(
        r#"insert into medicine(
                    medicine_name,
                    medicine_type,
                    medicine_location,
                    medicine_price,
                    medicine_import_date,
                    medicine_expire_date,
                    medicine_quantity,
                    medicine_prescripted
                )
                values($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
        medicine_name,
        medicine_type,
        medicine_location,
        medicine_price,
        medicine_import_date,
        medicine_expire_date,
        medicine_quantity,
        medicine_prescripted
    )
    .execute(&*DB)
    .await?;
    Ok(())
}
pub(crate) async fn delete_drug(id: i32) -> anyhow::Result<()> {
    query!(
        r#"delete from medicine 
            where medicine_id = $1
                "#,
        id,
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
pub(crate) async fn list_location() -> Result<Vec<String>> {
    Ok(query!(
        r#"select medicine_location
              from medicine
              group by medicine_location
              "#,
    )
    .fetch_all(&*DB)
    .await?
    .into_iter()
    .map(|obj| obj.medicine_location)
    .collect())
}
pub(crate) async fn edit_drug(
    medicine_id: i32,
    medicine_name: String,
    medicine_type: String,
    medicine_price: i32,
    medicine_quantity: i32,
    medicine_location: String,
    medicine_prescripted: bool,
) -> Result<()> {
    query!(
        r#"
            update medicine
                set medicine_name = $2,
                    medicine_type = $3,
                    medicine_price = $4,
                    medicine_quantity = $5,
                    medicine_location = $6,
                    medicine_prescripted = $7
                where medicine_id = $1
                "#,
        medicine_id,
        medicine_name,
        medicine_type,
        medicine_price,
        medicine_quantity,
        medicine_location,
        medicine_prescripted
    )
    .execute(&*DB)
    .await?;
    Ok(())
}

pub(crate) async fn match_user(username: &str, password: &str) -> Result<bool> {
    Ok(query!(
        r#"
        select staff_password from staff
        where staff_username = $1 and staff_password = $2
        "#,
        username,
        password
    )
    .fetch_optional(&*DB)
    .await?
    .is_some())
}
pub(crate) async fn get_customer_info(phone: &str) -> Result<(String, String)> {
    Ok(query!(
        r#"
        select customer_name, customer_address from bill
        where customer_phone ~* $1
        "#,
        phone
    )
    .fetch_one(&*DB)
    .await
    .map(|obj| (obj.customer_name, obj.customer_address))?)
}
pub(crate) async fn get_staff_info(username: &str) -> Result<(i32, String)> {
    Ok(query!(
        r#"
        select staff_id, staff_name from staff
        where staff_username = $1
        "#,
        username
    )
    .fetch_one(&*DB)
    .await
    .map(|obj| (obj.staff_id, obj.staff_name))?)
}

pub(crate) async fn new_bill() -> Result<i32> {
    Ok(query!(
        r#"
            insert into bill(staff_id, customer_phone, customer_name, customer_address)
            values (1, '0', 'Qua đường', 'Qua đường')
            returning bill_id;
        "#
    )
    .fetch_one(&*DB)
    .await?
    .bill_id)
}

pub(crate) async fn update_bill(
    bill_id: i32,
    bill_prescripted: bool,
    bill_done: bool,
    staff_id: i32,
    customer_phone: String,
    customer_name: String,
    customer_address: String,
) -> Result<()> {
    query!(
        r#"update bill
            set staff_id = $2,
            bill_prescripted = $3,
            bill_done = $4,
            customer_phone=$5,
            customer_name=$6,
            customer_address=$7
        where bill_id = $1
        "#,
        bill_id,
        staff_id,
        bill_prescripted,
        bill_done,
        customer_phone,
        customer_name,
        customer_address
    )
    .execute(&*DB)
    .await?;
    Ok(())
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct BillMedicineInfo {
    medicine_id: i32,
    medicine_quantity: i32,
    medicine_type: String,
    medicine_name: String,
    medicine_price: i32,
    medicine_location: String,
}

pub(crate) async fn list_bill_medicine(bill_id: i32) -> Result<Vec<BillMedicineInfo>> {
    Ok(find_drug("".to_owned(), "".to_string())
        .await?
        .into_iter()
        .map(|a| BillMedicineInfo {
            medicine_id: a.medicine_id,
            medicine_quantity: a.medicine_quantity,
            medicine_type: a.medicine_type,
            medicine_name: a.medicine_name,
            medicine_price: a.medicine_price,
            medicine_location: a.medicine_location,
        })
        .collect())
}

pub(crate) async fn add_bill_medicine(
    bill_id: i32,
    medicine_id: i32,
    medicine_price: i32,
    medicine_quantity: i32,
) -> Result<()> {
    query!(
        r#"
        insert into bill_medicine(bill_id, medicine_id, medicine_bill_price, medicine_bill_quantity)
        values ($1, $2, $3, $4)
        "#,
        bill_id,
        medicine_id,
        medicine_price,
        medicine_quantity
    )
    .execute(&*DB)
    .await?;
    Ok(())
}

#[derive(Serialize, Debug)]
pub(crate) struct BillSumary {
    bill_id: i32,
    bill_time: String,
    bill_prescripted: bool,
    bill_done: bool,
    staff_name: String,
    customer_name: String,
}

pub(crate) async fn all_bill(bill_done: bool) -> Result<Vec<BillSumary>> {
    Ok(query!(
        r#"select bill_id,
                bill_time,
                bill_prescripted,
                bill_done,
                staff_name,
                customer_name
            from bill
            join staff on bill.staff_id = staff.staff_id
            where bill_done = $1
        "#,
        bill_done
    )
    .fetch_all(&*DB)
    .await?
    .into_iter()
    .map(|obj| BillSumary {
        bill_id: obj.bill_id,
        bill_time: obj.bill_time.format("%D").to_string(),
        bill_prescripted: obj.bill_prescripted,
        bill_done: obj.bill_done,
        staff_name: obj.staff_name,
        customer_name: obj.customer_name,
    })
    .collect())
}
