mod database;
use std::collections::HashMap;

use async_std::{sync::Mutex, task::block_on};
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveTime, Offset, TimeZone, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};
use tera::{Context, Tera};
use tide::{Middleware, Redirect, Request, Response, Result};
use tide_tera::TideTeraExt;

static TERA: Lazy<Mutex<Tera>> =
    Lazy::new(|| Mutex::new(Tera::new("templates/**/*.html").unwrap()));

static LOGIN_CREDENTAL: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::from_iter(
        [("admin".to_owned(), "admin".to_owned())].into_iter(),
    ))
});

fn base_context(req: &Request<()>) -> Context {
    let mut context = Context::new();
    let username = &req.session().get::<String>("user").unwrap_or_default();
    context.insert("username", &username);
    context.insert("current_time", &Utc::now().to_rfc3339());

    context
}

async fn new_bill(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = base_context(&req);

    #[derive(Serialize, Debug, Clone)]
    struct MedicineInfo {
        id: String,
        code: String,
        name: String,
        r#type: String,
        price: u32,
        quantity: i32,
        import_date: String,
        location: String,
    }
    let val = MedicineInfo {
        id: 1.to_string(),
        code: "fads".into(),
        name: "Name".into(),
        r#type: "Name".into(),
        price: 12,
        quantity: 12,
        import_date: "date".into(),
        location: "Location".into(),
    };
    context.insert("bill_id", "1");
    context.insert("danhsach", &[&val; 20]);
    tera.render_response("new_bill.html", &context)
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

async fn staff(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    tera.render_response("staff.html", &context)
}
async fn bills(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    tera.render_response("bills.html", &context)
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
    tera.render_response("customer_info.html", &context)
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
