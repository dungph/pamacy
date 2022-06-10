mod database;

use async_std::{sync::Mutex, task::block_on};
use chrono::{DateTime, NaiveDate, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tide::{Middleware, Redirect, Request, Response, Result};
use tide_tera::TideTeraExt;

static TERA: Lazy<Mutex<Tera>> =
    Lazy::new(|| Mutex::new(Tera::new("templates/**/*.html").unwrap()));

fn base_context(req: &Request<()>) -> Context {
    let mut context = Context::new();
    let username = &req.session().get::<String>("user").unwrap_or_default();
    context.insert("staff_username", &username);
    context.insert("current_time", &Utc::now().to_rfc3339());
    context.insert(
        "staff_id",
        &req.session()
            .get::<String>("id")
            .unwrap_or_else(|| "Unknown".to_string()),
    );
    context.insert(
        "staff_name",
        &req.session()
            .get::<String>("name")
            .unwrap_or_else(|| "Unknown".to_string()),
    );
    context
}

async fn new_bill(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = base_context(&req);

    #[derive(Deserialize, Debug)]
    struct NewBill {
        bill_id: i32,
        staff_id: i32,
        date: NaiveDate,
        bill_prescripted: String,
        customer_phone: String,
        customer_name: String,
        medicine_id: i32,
        medicine_quantity: i32,
    }

    #[derive(Serialize, Debug, Clone)]
    struct BillInfo {
        staff_id: i32,
        staff_name: String,
        bill_id: i32,
        date: NaiveDate,
        bill_prescripted: String,
        customer_phone: String,
        customer_name: String,
    }

    if let Ok(new) = dbg!(req.query::<NewBill>()) {
        database::update_bill(
            new.bill_id,
            &new.bill_prescripted == "yes",
            false,
            new.staff_id,
            new.customer_phone.clone(),
            new.customer_name.clone(),
            String::new(),
        )
        .await?;

        context.insert("date", &Utc::today().format("%D").to_string());
        context.insert("staff_id", &new.staff_id);
        context.insert("bill_id", &new.bill_id);
        context.insert("date", &new.date);
        context.insert("customer_phone", &new.customer_phone);
        context.insert("customer_name", &new.customer_name);
        context.insert("bill_prescripted", &new.bill_prescripted);

        database::add_bill_medicine(new.bill_id, new.medicine_id, 0, new.medicine_quantity).await?;
    } else {
        let bill_id = database::new_bill().await?;
        context.insert("date", &Utc::today().format("%D").to_string());
        context.insert("bill_id", &bill_id);
        context.insert("staff_id", &1);
        context.insert("bill_prescripted", &"yes".to_string());
        context.insert("customer_name", &"Qua đường".to_string());
        context.insert("customer_phone", &"0".to_string());
        context.insert("date", &Utc::today().naive_utc()); //.format("%D").to_string(),
    }
    context.insert("danhsach", &database::list_bill_medicine(1).await?);
    tera.render_response("bill/new_bill.html", &context)
}

async fn manage_page(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = base_context(&req);

    #[derive(Deserialize, Debug)]
    struct FindForm {
        find_medicine_name: String,
        find_medicine_type: String,
    }

    #[derive(Deserialize, Debug)]
    struct MedicineAddForm {
        //medicine_add: String,
        //new_medicine_id: i32,
        new_medicine_expire_date: NaiveDate,
        new_medicine_price: i32,
        new_medicine_name: String,
        new_medicine_type: String,
        new_medicine_quantity: i32,
        new_medicine_location: String,
        new_medicine_prescripted: String,
    }

    #[derive(Deserialize, Debug)]
    struct MedicineEditForm {
        medicine_edit: String,
        new_medicine_id: i32,
        new_medicine_name: String,
        new_medicine_price: i32,
        new_medicine_type: String,
        new_medicine_quantity: i32,
        new_medicine_location: String,
        new_medicine_prescripted: String,
    }
    #[derive(Deserialize, Debug)]
    struct MedicineDeleteForm {
        medicine_delete: String,
        new_medicine_id: i32,
    }

    if let Ok(find_form) = req.query::<FindForm>() {
        context.insert(
            "display",
            &database::find_drug(find_form.find_medicine_name, find_form.find_medicine_type)
                .await?,
        );
    } else if let Ok(add_form) = dbg!(req.query::<MedicineAddForm>()) {
        database::add_drug(
            add_form.new_medicine_name,
            add_form.new_medicine_type,
            add_form.new_medicine_location,
            add_form.new_medicine_price,
            add_form.new_medicine_quantity,
            Utc::now(),
            DateTime::from_utc(add_form.new_medicine_expire_date.and_hms(0, 0, 0), Utc),
            add_form.new_medicine_prescripted.as_str() == "yes",
        )
        .await?;
        return Ok(Redirect::new("/manage").into());
    } else if let Ok(delete_form) = dbg!(req.query::<MedicineDeleteForm>()) {
        database::delete_drug(delete_form.new_medicine_id).await?;
        return Ok(Redirect::new("/manage").into());
    } else if let Ok(edit_form) = dbg!(req.query::<MedicineEditForm>()) {
        //database::edit_drug(edit_form.new_medicine_id, edit_form.new_medicine_name ).await?;
        database::edit_drug(
            edit_form.new_medicine_id,
            edit_form.new_medicine_name,
            edit_form.new_medicine_type,
            edit_form.new_medicine_price,
            edit_form.new_medicine_quantity,
            edit_form.new_medicine_location,
            edit_form.new_medicine_prescripted.as_str() == "yes",
        )
        .await?;
        return Ok(Redirect::new("/manage").into());
    } else {
        context.insert(
            "display",
            &database::find_drug("".to_string(), "".to_string()).await?,
        );
    };

    context.insert("medicine_type_list", &database::list_drug_type().await?);
    context.insert("medicine_location_list", &database::list_location().await?);

    context.insert("new_medicine_id", &1);
    tera.render_response("manage/manage.html", &context)
}

async fn bills(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    context.insert("list_bill_sumary", &database::all_bill(true).await?);

    tera.render_response("bill/bills.html", &context)
}
async fn staff(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    tera.render_response("staff.html", &context)
}
async fn finance(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    tera.render_response("finance.html", &context)
}
async fn customer(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    tera.render_response("customer.html", &context)
}
async fn statistic(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    tera.render_response("statistic.html", &context)
}
async fn index(req: Request<()>) -> Result<Response> {
    if req.session().get::<String>("user").is_some() {
        Ok(Redirect::new("/manage").into())
    } else {
        let mut tera = TERA.lock().await;
        tera.full_reload()?;
        let mut context = Context::new();

        tera.render_response("index.html", &context)
    }
}

async fn login(mut req: Request<()>) -> Result<Response> {
    #[derive(Deserialize, Debug)]
    #[serde(tag = "submit")]
    #[serde(rename_all = "lowercase")]
    enum Session {
        Login { username: String, password: String },
        Logout { username: String },
    }
    let form: Session = dbg!(req.body_form().await?);
    match form {
        Session::Login { username, password } => {
            if database::match_user(username.as_str(), password.as_str()).await? {
                let res: Response = Redirect::new("/manage").into();
                let (id, name) = database::get_staff_info(username.as_str()).await?;
                req.session_mut().insert("id", id)?;
                req.session_mut().insert("name", name)?;
                req.session_mut().insert("user", username)?;
                Ok(res)
            } else {
                let mut tera = TERA.lock().await;
                tera.full_reload()?;
                let mut context = Context::new();
                context.insert("msg", "Wrong username or password");
                tera.render_response("index.html", &context)
            }
        }
        Session::Logout { username } => {
            req.session_mut().destroy();
            Ok(Redirect::new("/").into())
        }
    }
}
fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    let mut server = tide::with_state(());

    server.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        rand::random::<[u8; 32]>().as_ref(),
    ));
    struct Auth;
    #[async_trait::async_trait]
    impl Middleware<()> for Auth {
        async fn handle(&self, req: Request<()>, next: tide::Next<'_, ()>) -> Result<Response> {
            if req.session().get::<String>("user").is_some() {
                Ok(next.run(req).await)
            } else {
                Ok(Redirect::new("/").into())
            }
        }
    }

    server.at("/").reset_middleware().get(index);
    server.at("/login").post(login);
    server.at("/assert").serve_dir("assert").unwrap();

    server.at("/new_bill").with(Auth).get(new_bill);
    server.at("/manage").with(Auth).get(manage_page);
    server.at("/staff").with(Auth).get(staff);
    server.at("/bills").with(Auth).get(bills);
    server.at("/finance").with(Auth).get(finance);
    server.at("/customer").with(Auth).get(customer);
    server.at("/statistic").with(Auth).get(statistic);

    block_on(database::migrate())?;
    Ok(block_on(server.listen("0.0.0.0:8080"))?)
}
