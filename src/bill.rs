use serde::{Deserialize, Serialize};
use tide::{Redirect, Request, Response, Result};
use tide_tera::TideTeraExt;

use crate::{base_context, database, TERA};

pub(crate) async fn new_bill(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = base_context(&req);

    #[derive(Serialize, Deserialize, Debug)]
    struct NewBill {
        bill_id: Option<i32>,
    }

    let NewBill { bill_id } = req.query()?;
    context.insert("bill_id", &bill_id);

    let bill_info = if let Some(bill_id) = bill_id {
        database::get_bill(bill_id).await?
    } else {
        let info =
            database::new_bill(req.session().get::<String>("username").unwrap().as_str()).await?;
        return Ok(Redirect::new(format!("/new_bill?bill_id={}", info.bill_id)).into());
    };

    context.insert("bill_id", &bill_info.bill_id);
    context.insert("bill_prescripted", &bill_info.bill_prescripted);
    context.insert(
        "danhsach",
        &database::list_bill_medicine(bill_info.bill_id).await?,
    );
    context.insert(
        "bill_amount",
        &database::bill_amount(bill_info.bill_id).await?,
    );
    tera.render_response("bill/new_bill.html", &context)
}

pub(crate) async fn edit_info(req: Request<()>) -> Result<Response> {
    #[derive(Deserialize, Debug)]
    struct NewBill {
        bill_id: i32,
        staff_username: String,
        bill_prescripted: String,
    }
    let new_info = dbg!(req.query::<NewBill>()?);
    database::update_bill(
        new_info.bill_id,
        new_info.bill_prescripted == "yes",
        new_info.staff_username,
    )
    .await?;

    Ok(Redirect::new(format!("/new_bill?bill_id={}", new_info.bill_id)).into())
}
pub(crate) async fn add_medicine(req: Request<()>) -> Result<Response> {
    #[derive(Deserialize, Debug)]
    struct MedicineBill {
        bill_id: i32,
        medicine_code: String,
    }

    let new_info = req.query::<MedicineBill>()?;
    database::add_bill_medicine(new_info.bill_id, new_info.medicine_code).await?;
    Ok(Redirect::new(format!("/new_bill?bill_id={}", new_info.bill_id)).into())
}
pub(crate) async fn edit_medicine(req: Request<()>) -> Result<Response> {
    #[derive(Deserialize, Debug)]
    struct EditMedicine {
        bill_id: i32,
        medicine_code: String,
        medicine_quantity: i32,
    }

    let new_info = req.query::<EditMedicine>()?;
    database::edit_bill_medicine(
        new_info.bill_id,
        &new_info.medicine_code,
        new_info.medicine_quantity,
    )
    .await?;
    Ok(Redirect::new(format!("/new_bill?bill_id={}", new_info.bill_id)).into())
}
pub(crate) async fn complete(req: Request<()>) -> Result<Response> {
    #[derive(Deserialize, Debug, Clone)]
    struct BillInfo {
        bill_id: i32,
        customer_phone: String,
        customer_name: String,
    }

    let new_info = req.query::<BillInfo>()?;
    database::complete_bill(
        new_info.bill_id,
        new_info.customer_name,
        new_info.customer_phone,
    )
    .await?;
    Ok(Redirect::new("/bills").into())
}
