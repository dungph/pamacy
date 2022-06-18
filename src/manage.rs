use serde::Deserialize;
use tide::{Redirect, Request, Response, Result};
use tide_tera::TideTeraExt;

use crate::{base_context, database, TERA};

pub(crate) async fn manage_page(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = base_context(&req);

    #[derive(Deserialize, Debug)]
    struct FindForm {
        find_medicine_name: String,
        find_medicine_type: String,
    }

    if let Ok(find_form) = req.query::<FindForm>() {
        context.insert(
            "display",
            &database::find_drug(
                find_form.find_medicine_name.as_str(),
                find_form.find_medicine_type.as_str(),
            )
            .await?,
        );
    } else {
        context.insert("display", &database::find_drug("", "").await?);
    };

    context.insert("medicine_type_list", &database::list_drug_type().await?);
    context.insert("medicine_location_list", &database::list_location().await?);

    context.insert("new_medicine_id", &1);
    tera.render_response("manage/manage.html", &context)
}

pub(crate) async fn add_medicine(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);

    #[derive(Deserialize, Debug)]
    struct MedicineAddForm {
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
        medicine_prescripted: String,
    }
    if let Ok(add_form) = dbg!(req.query::<MedicineAddForm>()) {
        database::add_drug(
            add_form.medicine_code,
            add_form.medicine_name,
            add_form.medicine_price,
            add_form.medicine_register,
            add_form.medicine_content,
            add_form.medicine_active_ingredients,
            add_form.medicine_pack_form,
            add_form.medicine_group,
            add_form.medicine_route,
            add_form.medicine_quantity,
            add_form.medicine_location,
            add_form.medicine_prescripted.as_str() == "yes",
        )
        .await?;
        return Ok(Redirect::new("/manage").into());
    } else {
    }
    tera.render_response("bill/bills.html", &context)
}

pub(crate) async fn edit_medicine(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    //context.insert("list_bill_sumary", &database::all_bill(true).await?);

    #[derive(Deserialize, Debug)]
    struct MedicineEditForm {
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
        medicine_prescripted: String,
    }

    if let Ok(edit_form) = dbg!(req.query::<MedicineEditForm>()) {
        //database::edit_drug(edit_form.new_medicine_id, edit_form.new_medicine_name ).await?;
        database::edit_drug(
            edit_form.medicine_code,
            edit_form.medicine_name,
            edit_form.medicine_price,
            edit_form.medicine_register,
            edit_form.medicine_content,
            edit_form.medicine_active_ingredients,
            edit_form.medicine_pack_form,
            edit_form.medicine_group,
            edit_form.medicine_route,
            edit_form.medicine_quantity,
            edit_form.medicine_location,
            edit_form.medicine_prescripted.as_str() == "yes",
        )
        .await?;
        return Ok(Redirect::new("/manage").into());
    } else {
    }
    tera.render_response("bill/bills.html", &context)
}
