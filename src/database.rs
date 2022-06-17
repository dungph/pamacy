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
        insert into staff(staff_fullname, staff_username, staff_password)
        values ('Administrator', 'admin', 'admin')
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

pub(crate) async fn find_drug(name: &str, drug_type: &str) -> Result<Vec<ManageMedicineTemplate>> {
    Ok(query_as!(
        ManageMedicineTemplate,
        r#"select 
                medicine_id,
                medicine_name,
                medicine_type,
                medicine_prescripted,
                medicine_price,
                medicine_quantity,
                medicine_location
            from medicine
            where (medicine_name ~* $1 and medicine_type ~* $2)
            order by medicine_id asc;
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

#[derive(Serialize)]
pub(crate) struct Customer {
    customer_phone: String,
    customer_name: String,
    customer_address: String,
}
pub(crate) async fn all_customer() -> Result<Vec<Customer>> {
    Ok(query!(
        r#"select customer_name, customer_phone, customer_address from bill
            order by customer_name asc;
            "#
    )
    .fetch_all(&*DB)
    .await?
    .into_iter()
    .map(|obj| Customer {
        customer_phone: obj.customer_phone,
        customer_name: obj.customer_name,
        customer_address: obj.customer_address,
    })
    .collect())
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
pub(crate) async fn get_staff_name(username: &str) -> Result<String> {
    Ok(query!(
        r#"
        select staff_fullname from staff
        where staff_username = $1
        "#,
        username
    )
    .fetch_one(&*DB)
    .await
    .map(|obj| obj.staff_fullname)?)
}

pub(crate) async fn update_bill(
    bill_id: i32,
    bill_prescripted: bool,
    staff_username: String,
) -> Result<()> {
    query!(
        r#"
        update bill
            set staff_username = $2,
            bill_prescripted = $3
        where bill_id = $1
        "#,
        bill_id,
        staff_username,
        bill_prescripted,
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
    Ok(query!(
        r#"select 
                medicine.medicine_id,
                medicine_bill_quantity,
                medicine_bill_price,
                medicine_location,
                medicine_type,
                medicine_name
            from medicine_bill
            join medicine on medicine.medicine_id = medicine_bill.medicine_id            
            where bill_id = $1
            order by medicine_id asc;
            "#,
        bill_id
    )
    .fetch_all(&*DB)
    .await?
    .into_iter()
    .map(|a| BillMedicineInfo {
        medicine_id: a.medicine_id,
        medicine_quantity: a.medicine_bill_quantity,
        medicine_type: a.medicine_type,
        medicine_name: a.medicine_name,
        medicine_price: a.medicine_bill_price,
        medicine_location: a.medicine_location,
    })
    .collect())
}

pub(crate) async fn new_bill(username: &str) -> Result<i32> {
    Ok(query!(
        r#"
            insert into bill(staff_username, customer_phone, customer_name, customer_address)
            values ($1, '0', 'Qua đường', 'Qua đường')
            returning bill_id;
        "#,
        username
    )
    .fetch_one(&*DB)
    .await?
    .bill_id)
}

pub(crate) async fn add_bill_medicine(
    bill_id: i32,
    medicine_id: i32,
    medicine_price: i32,
    medicine_quantity: i32,
) -> Result<()> {
    query!(
        r#"
        insert into medicine_bill(bill_id, medicine_id, medicine_bill_price, medicine_bill_quantity)
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
pub(crate) async fn edit_bill_medicine(
    bill_id: i32,
    medicine_id: i32,
    medicine_price: i32,
    medicine_quantity: i32,
) -> Result<()> {
    query!(
        r#"
        update medicine_bill
        set medicine_bill_price = $3,
            medicine_bill_quantity = $4
        where bill_id = $1 and medicine_id = $2;
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

pub(crate) async fn complete_bill(
    bill_id: i32,
    customer_name: String,
    customer_phone: String,
) -> Result<()> {
    query!(
        r#"
        CALL reduce_medicine_quantity($1);
        "#,
        bill_id
    )
    .execute(&*DB)
    .await?;

    query!(
        r#"
        update bill set
            customer_name = $2,
            customer_phone = $3,
            bill_done = true
        where
            bill_id = $1;
        "#,
        bill_id,
        customer_name,
        customer_phone
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
    bill_amount: i64,
}

pub(crate) async fn all_bill(bill_done: bool) -> Result<Vec<BillSumary>> {
    Ok(query!(
        r#"select bill.bill_id,
                bill_time,
                bill_prescripted,
                bill_done,
                staff_fullname,
                customer_name,
                amount_tb.amount
            from bill
            join staff on bill.staff_username = staff.staff_username
            join ( select bill_id, SUM(medicine_bill_price * medicine_bill_quantity) as amount from medicine_bill
                group by bill_id
                ) amount_tb on amount_tb.bill_id = bill.bill_id
            where bill_done = $1
            order by bill_id asc;
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
        staff_name: obj.staff_fullname,
        customer_name: obj.customer_name,
        bill_amount: obj.amount.unwrap_or(0)
    })
    .collect())
}

pub(crate) async fn bill_amount(bill_id: i32) -> Result<Option<i64>> {
    Ok(query!(
        r#"
        select SUM(medicine_bill_price * medicine_bill_quantity) as amount from medicine_bill
            where bill_id = $1
            group by bill_id
        "#,
        bill_id
    )
    .fetch_optional(&*DB)
    .await?
    .map(|o| o.amount)
    .flatten())
}
#[derive(Serialize)]
pub(crate) struct StaffInfo {
    staff_fullname: String,
    staff_username: String,
}
pub(crate) async fn all_staff() -> Result<Vec<StaffInfo>> {
    Ok(query!(
        r#"select staff_username, staff_fullname from staff
            order by staff_username asc;
            "#
    )
    .fetch_all(&*DB)
    .await?
    .into_iter()
    .map(|obj| StaffInfo {
        staff_fullname: obj.staff_fullname,
        staff_username: obj.staff_username,
    })
    .collect())
}
