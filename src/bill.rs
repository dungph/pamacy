use serde::Deserialize;
use tide::{Redirect, Request, Response, Result};
use tide_tera::TideTeraExt;

use crate::{base_context, TERA};

pub(crate) async fn new_bill(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = base_context(&req);

    #[derive(Deserialize, Debug)]
    struct NewBill {
        bill_id: Option<i32>,
        staff_username: Option<String>,
        bill_prescripted: Option<String>,
    }

    //if let Ok(new) = dbg!(req.query::<MedicineBill>()) {
    //    context.insert("staff_username", &new.staff_username);
    //    context.insert("bill_id", &new.bill_id);
    //    context.insert("customer_phone", &0);
    //    context.insert("customer_name", &"Unknown");
    //    context.insert("bill_prescripted", &new.bill_prescripted);
    //    database::update_bill(
    //        new.bill_id,
    //        new.bill_prescripted.as_str() == "yes",
    //        new.staff_username,
    //    )
    //    .await?;
    //    database::add_bill_medicine(
    //        new.bill_id,
    //        new.medicine_id,
    //        new.medicine_price,
    //        new.medicine_quantity,
    //    )
    //    .await?;
    //    context.insert(
    //        "danhsach",
    //        &database::list_bill_medicine(new.bill_id).await?,
    //    );
    //    context.insert(
    //        "bill_amount",
    //        &database::bill_amount(new.bill_id).await?.unwrap_or(0),
    //    );
    //} else if let Ok(new) = dbg!(req.query::<EditMedicineBill>()) {
    //    context.insert("staff_username", &new.staff_username);
    //    context.insert("bill_id", &new.bill_id);
    //    context.insert("customer_phone", &0);
    //    context.insert("customer_name", &"Unknown");
    //    context.insert("bill_prescripted", &new.bill_prescripted);

    //    database::edit_bill_medicine(
    //        new.bill_id,
    //        new.edit_medicine_id,
    //        new.edit_medicine_price,
    //        new.edit_medicine_quantity,
    //    )
    //    .await?;
    //    context.insert(
    //        "danhsach",
    //        &database::list_bill_medicine(new.bill_id).await?,
    //    );
    //    context.insert(
    //        "bill_amount",
    //        &database::bill_amount(new.bill_id).await?.unwrap_or(0),
    //    );
    //} else if let Ok(new) = dbg!(req.query::<NewBill>()) {
    //    context.insert("staff_username", &new.staff_username);
    //    context.insert("bill_id", &new.bill_id);
    //    context.insert("customer_phone", &0);
    //    context.insert("customer_name", &"Unknown");
    //    context.insert("bill_prescripted", &new.bill_prescripted);
    //    database::update_bill(
    //        new.bill_id,
    //        new.bill_prescripted.as_str() == "yes",
    //        new.staff_username,
    //    )
    //    .await?;

    //    context.insert(
    //        "danhsach",
    //        &database::list_bill_medicine(new.bill_id).await?,
    //    );
    //    context.insert(
    //        "bill_amount",
    //        &database::bill_amount(new.bill_id).await?.unwrap_or(0),
    //    );
    //} else if let Ok(new) = dbg!(req.query::<BillInfo>()) {
    //    database::complete_bill(new.bill_id, new.customer_name, new.customer_phone).await?;
    //    return Ok(Redirect::new("/bills").into());
    //} else {
    //    let bill_id = database::new_bill(&req.session().get::<String>("username").unwrap()).await?;
    //    context.insert("bill_id", &bill_id);
    //    context.insert("staff_id", &1);
    //    context.insert("bill_prescripted", &"yes".to_string());
    //    context.insert("customer_name", &"Qua đường".to_string());
    //    context.insert("customer_phone", &"0".to_string());
    //    context.insert(
    //        "danhsach",
    //        &database::list_bill_medicine(i32::max_value()).await?,
    //    );
    //    context.insert("bill_amount", &0);
    //}
    tera.render_response("bill/new_bill.html", &context)
}

pub(crate) async fn edit_info(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    #[derive(Deserialize, Debug)]
    struct NewBill {
        bill_id: i32,
        staff_username: String,
        bill_prescripted: String,
    }

    let mut context = base_context(&req);
    tera.render_response("bill/new_bill.html", &context)
}
pub(crate) async fn add_medicine(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    #[derive(Deserialize, Debug)]
    struct EditMedicineBill {
        bill_id: i32,
        medicine_id: i32,
        medicine_quantity: i32,
    }

    let mut context = base_context(&req);
    tera.render_response("bill/new_bill.html", &context)
}
pub(crate) async fn edit_price(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    #[derive(Deserialize, Debug)]
    struct EditMedicineBill {
        bill_id: i32,
        medicine_id: i32,
        medicine_quantity: i32,
    }

    let mut context = base_context(&req);
    tera.render_response("bill/new_bill.html", &context)
}
pub(crate) async fn edit_quantity(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    #[derive(Deserialize, Debug)]
    struct MedicineBill {
        bill_id: i32,
        medicine_id: i32,
        medicine_quantity: i32,
    }

    let mut context = base_context(&req);
    tera.render_response("bill/new_bill.html", &context)
}
pub(crate) async fn complete(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    #[derive(Deserialize, Debug, Clone)]
    struct BillInfo {
        bill_id: i32,
        customer_phone: String,
        customer_name: String,
    }

    let mut context = base_context(&req);
    tera.render_response("bill/new_bill.html", &context)
}
