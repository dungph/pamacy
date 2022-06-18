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
        insert into staff(staff_fullname, staff_username, staff_password, staff_is_manager, staff_is_seller)
        values ('Administrator', 'admin', 'admin', true, true)
        on conflict (staff_username) do nothing;
        "#
    )
    .execute(&*DB)
    .await?;
    Ok(())
}

#[derive(Serialize, Debug)]
pub(crate) struct MedicineInfo {
    medicine_code: String,
    medicine_name: String,
    medicine_price: i32,
    medicine_register: String,
    medicine_content: String,
    medicine_active_ingredients: String,
    medicine_pack_form: String,
    medicine_group: String,
    medicine_route: String,
    medicine_quantity: Option<i64>,
    medicine_location: String,
    medicine_prescripted: bool,
}

pub(crate) async fn find_drug(name: &str, drug_type: &str) -> Result<Vec<MedicineInfo>> {
    Ok(query_as!(
        MedicineInfo,
        r#"
            select medicine.medicine_code as "medicine_code!",
                medicine_name as "medicine_name!",
                medicine_price as "medicine_price!",
                medicine_register as "medicine_register!",
                medicine_content as "medicine_content!",
                medicine_active_ingredients as "medicine_active_ingredients!",
                medicine_prescripted as "medicine_prescripted!",
                medicine_pack_form as "medicine_pack_form!",
                medicine_group as "medicine_group!",
                medicine_route as "medicine_route!",
                medicine_locations as "medicine_location!",
                SUM(medicine_inventory_quantity) as medicine_quantity
            from medicine_info
            join medicine on medicine.medicine_code = medicine_info.medicine_code
            join medicine_inventory_bill on medicine_inventory_bill.medicine_id = medicine.medicine_id
            where (medicine_name ~* $1 and medicine_group ~* $2)
            group by (
                medicine.medicine_code,
                medicine_name, 
                medicine_price,
                medicine_register,
                medicine_content,
                medicine_active_ingredients,
                medicine_prescripted,
                medicine_pack_form,
                medicine_group,
                medicine_locations,
                medicine_route
                )
            "#,
        name,
        drug_type,
    )
    .fetch_all(&*DB)
    .await?)
}

pub(crate) async fn add_drug(
    staff_username: &str,
    medicine_code: String,
    medicine_name: String,
    medicine_price: i32,
    medicine_register: String,
    medicine_content: String,
    medicine_active_ingredients: String,
    medicine_pack_form: String,
    medicine_group: String,
    medicine_route: String,
    medicine_quantity: i32,
    medicine_location: String,
    medicine_prescripted: bool,
) -> Result<()> {
    query!(
        r#"insert into medicine_info(
                medicine_code,
                medicine_name, 
                medicine_price,
                medicine_register,
                medicine_content,
                medicine_active_ingredients,
                medicine_prescripted,
                medicine_pack_form,
                medicine_group,
                medicine_locations,
                medicine_route
            )
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        on conflict (medicine_code) do nothing;
            "#,
        medicine_code,
        medicine_name,
        medicine_price,
        medicine_register,
        medicine_content,
        medicine_active_ingredients,
        medicine_prescripted,
        medicine_pack_form,
        medicine_group,
        medicine_location,
        medicine_route
    )
    .execute(&*DB)
    .await?;

    let medicine_id = query!(
        r#"insert into medicine(
                    medicine_code
                )
                values($1)
                returning medicine_id;
                "#,
        medicine_code,
    )
    .fetch_one(&*DB)
    .await?
    .medicine_id;

    let bill_id = query!(
        r#"
        insert into inventory_bill(inventory_bill_complete, staff_username)
        values (true, $1)
        returning inventory_bill_id
        "#,
        staff_username
    )
    .fetch_one(&*DB)
    .await?
    .inventory_bill_id;
    query!(r#"
        insert into medicine_inventory_bill(inventory_bill_id, medicine_id, medicine_inventory_price, medicine_inventory_quantity)
        values ($1, $2, $3, $4)
        "#,
        bill_id,
        medicine_id,
        medicine_price,
        medicine_quantity
        ).execute(&*DB).await?;
    Ok(())
}

pub(crate) async fn edit_drug(
    medicine_code: String,
    medicine_name: String,
    medicine_price: i32,
    medicine_register: String,
    medicine_content: String,
    medicine_active_ingredients: String,
    medicine_pack_form: String,
    medicine_group: String,
    medicine_route: String,
    medicine_quantity: i32,
    medicine_location: String,
    medicine_prescripted: bool,
) -> Result<()> {
    query!(
        r#"
            update medicine_info
                set medicine_name = $2, 
                medicine_price = $3,
                medicine_register = $4,
                medicine_content = $5,
                medicine_active_ingredients = $6,
                medicine_prescripted = $7,
                medicine_pack_form = $8,
                medicine_group = $9,
                medicine_locations = $10,
                medicine_route = $11
                where medicine_code = $1
                "#,
        medicine_code,
        medicine_name,
        medicine_price,
        medicine_register,
        medicine_content,
        medicine_active_ingredients,
        medicine_prescripted,
        medicine_pack_form,
        medicine_group,
        medicine_location,
        medicine_route
    )
    .execute(&*DB)
    .await?;
    Ok(())
}

pub(crate) async fn list_drug_type() -> anyhow::Result<Vec<String>> {
    Ok(query!(
        r#"select medicine_group as "medicine_type!"
              from medicine_info
              group by medicine_group
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
        r#"select medicine_locations
              from medicine_info
              group by medicine_locations
              "#,
    )
    .fetch_all(&*DB)
    .await?
    .into_iter()
    .map(|obj| obj.medicine_locations)
    .collect())
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
        r#"select customer_name, customer_phone, customer_address from customer
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
//pub(crate) async fn get_customer_info(phone: &str) -> Result<(String, String)> {
//    Ok(query!(
//        r#"
//        select customer_name, customer_address from bill
//        where customer_phone ~* $1
//        "#,
//        phone
//    )
//    .fetch_one(&*DB)
//    .await
//    .map(|obj| (obj.customer_name, obj.customer_address))?)
//}
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

#[derive(Serialize, Debug)]
pub(crate) struct BillInfo {
    pub bill_id: i32,
    pub staff_username: String,
    pub bill_prescripted: bool,
}
pub(crate) async fn new_bill(username: &str) -> Result<BillInfo> {
    let inventory_bill_id = query!(
        r#"
        insert into inventory_bill(staff_username)
        values ($1)
        returning inventory_bill_id;
        "#,
        username
    )
    .fetch_one(&*DB)
    .await?
    .inventory_bill_id;

    let bill_id = query!(
        r#"
        insert into sell_bill(staff_username, inventory_bill_id, is_prescripted)
        values ($1, $2, $3)
        returning sell_bill_id;
        "#,
        username,
        inventory_bill_id,
        false,
    )
    .fetch_one(&*DB)
    .await?
    .sell_bill_id;

    Ok(BillInfo {
        bill_id,
        bill_prescripted: false,
        staff_username: username.to_string(),
    })
}

pub(crate) async fn get_bill(bill_id: i32) -> Result<BillInfo> {
    Ok(query_as!(
        BillInfo,
        r#"
        select sell_bill_id as bill_id, staff_username , is_prescripted as "bill_prescripted!"
        from sell_bill
        where sell_bill_id = $1
        "#,
        bill_id
    )
    .fetch_one(&*DB)
    .await?)
}

pub(crate) async fn update_bill(
    bill_id: i32,
    bill_prescripted: bool,
    staff_username: String,
) -> Result<()> {
    let inventory_bill_id = query!(
        r#"
        update sell_bill
        set staff_username = $2, is_prescripted = $3
        where sell_bill_id = $1
        returning inventory_bill_id
        "#,
        bill_id,
        staff_username,
        bill_prescripted
    )
    .fetch_one(&*DB)
    .await?
    .inventory_bill_id;

    query!(
        r#"
        update inventory_bill
        set staff_username = $2
        where inventory_bill_id = $1
        "#,
        inventory_bill_id,
        staff_username,
    )
    .execute(&*DB)
    .await?;
    Ok(())
}

pub(crate) async fn list_bill_medicine(bill_id: i32) -> Result<Vec<MedicineInfo>> {
    Ok(query_as!(
        MedicineInfo,
        r#"
            select medicine.medicine_code as "medicine_code!",
                medicine_name as "medicine_name!",
                medicine_price as "medicine_price!",
                medicine_register as "medicine_register!",
                medicine_content as "medicine_content!",
                medicine_active_ingredients as "medicine_active_ingredients!",
                medicine_prescripted as "medicine_prescripted!",
                medicine_pack_form as "medicine_pack_form!",
                medicine_group as "medicine_group!",
                medicine_route as "medicine_route!",
                medicine_locations as "medicine_location!",
                SUM(medicine_inventory_quantity) as medicine_quantity
            from medicine_info
            join medicine on medicine.medicine_code = medicine_info.medicine_code
            join medicine_inventory_bill on medicine_inventory_bill.medicine_id = medicine.medicine_id
            join sell_bill on sell_bill.inventory_bill_id = medicine_inventory_bill.inventory_bill_id
            where (sell_bill_id = $1)
            group by (
                medicine.medicine_code,
                medicine_name, 
                medicine_price,
                medicine_register,
                medicine_content,
                medicine_active_ingredients,
                medicine_prescripted,
                medicine_pack_form,
                medicine_group,
                medicine_locations,
                medicine_route
                )
            "#,
            bill_id
    )
    .fetch_all(&*DB)
    .await?)
}

pub(crate) async fn bill_amount(bill_id: i32) -> Result<Option<i64>> {
    Ok(query!(
        r#"
        select SUM(medicine_inventory_price * medicine_inventory_quantity) as amount from medicine_inventory_bill
        join sell_bill on sell_bill.inventory_bill_id = medicine_inventory_bill.inventory_bill_id
            where sell_bill_id = $1
            group by sell_bill_id
        "#,
        bill_id
    )
    .fetch_optional(&*DB)
    .await?
    .and_then(|o| o.amount))
}

//pub(crate) async fn bill_info()

//pub(crate) async fn new_bill(username: &str) -> Result<i32> {
//    Ok(query!(
//        r#"
//            insert into bill(staff_username, customer_phone, customer_name, customer_address)
//            values ($1, '0', 'Qua đường', 'Qua đường')
//            returning bill_id;
//        "#,
//        username
//    )
//    .fetch_one(&*DB)
//    .await?
//    .bill_id)
//}
//
//pub(crate) async fn add_bill_medicine(
//    bill_id: i32,
//    medicine_id: i32,
//    medicine_price: i32,
//    medicine_quantity: i32,
//) -> Result<()> {
//    query!(
//        r#"
//        insert into medicine_bill(bill_id, medicine_id, medicine_bill_price, medicine_bill_quantity)
//        values ($1, $2, $3, $4)
//        "#,
//        bill_id,
//        medicine_id,
//        medicine_price,
//        medicine_quantity
//    )
//    .execute(&*DB)
//    .await?;
//    Ok(())
//}
//pub(crate) async fn edit_bill_medicine(
//    bill_id: i32,
//    medicine_id: i32,
//    medicine_price: i32,
//    medicine_quantity: i32,
//) -> Result<()> {
//    query!(
//        r#"
//        update medicine_bill
//        set medicine_bill_price = $3,
//            medicine_bill_quantity = $4
//        where bill_id = $1 and medicine_id = $2;
//        "#,
//        bill_id,
//        medicine_id,
//        medicine_price,
//        medicine_quantity
//    )
//    .execute(&*DB)
//    .await?;
//    Ok(())
//}

//pub(crate) async fn complete_bill(
//    bill_id: i32,
//    customer_name: String,
//    customer_phone: String,
//) -> Result<()> {
//    query!(
//        r#"
//        CALL reduce_medicine_quantity($1);
//        "#,
//        bill_id
//    )
//    .execute(&*DB)
//    .await?;
//
//    query!(
//        r#"
//        update bill set
//            customer_name = $2,
//            customer_phone = $3,
//            bill_done = true
//        where
//            bill_id = $1;
//        "#,
//        bill_id,
//        customer_name,
//        customer_phone
//    )
//    .execute(&*DB)
//    .await?;
//    Ok(())
//}

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

//pub(crate) async fn all_bill(bill_done: bool) -> Result<Vec<BillSumary>> {
//    Ok(query!(
//        r#"select bill.bill_id,
//                bill_time,
//                bill_prescripted,
//                bill_done,
//                staff_fullname,
//                customer_name,
//                amount_tb.amount
//            from bill
//            join staff on bill.staff_username = staff.staff_username
//            join ( select bill_id, SUM(medicine_bill_price * medicine_bill_quantity) as amount from medicine_bill
//                group by bill_id
//                ) amount_tb on amount_tb.bill_id = bill.bill_id
//            where bill_done = $1
//            order by bill_id asc;
//        "#,
//        bill_done
//    )
//    .fetch_all(&*DB)
//    .await?
//    .into_iter()
//    .map(|obj| BillSumary {
//        bill_id: obj.bill_id,
//        bill_time: obj.bill_time.format("%D").to_string(),
//        bill_prescripted: obj.bill_prescripted,
//        bill_done: obj.bill_done,
//        staff_name: obj.staff_fullname,
//        customer_name: obj.customer_name,
//        bill_amount: obj.amount.unwrap_or(0)
//    })
//    .collect())
//}

//pub(crate) async fn bill_amount(bill_id: i32) -> Result<Option<i64>> {
//    Ok(query!(
//        r#"
//        select SUM(medicine_bill_price * medicine_bill_quantity) as amount from medicine_bill
//            where bill_id = $1
//            group by bill_id
//        "#,
//        bill_id
//    )
//    .fetch_optional(&*DB)
//    .await?
//    .map(|o| o.amount)
//    .flatten())
//}
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
