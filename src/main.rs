mod bill;
mod database;
mod manage;

use async_std::{sync::Mutex, task::block_on};
use chrono::{DateTime, NaiveDate, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tide::{Middleware, Redirect, Request, Response, Result};
use tide_tera::TideTeraExt;

pub static TERA: Lazy<Mutex<Tera>> =
    Lazy::new(|| Mutex::new(Tera::new("templates/**/*.html").unwrap()));

pub(crate) fn base_context(req: &Request<()>) -> Context {
    let mut context = Context::new();
    let username = &req.session().get::<String>("username").unwrap_or_default();
    context.insert("staff_username", &username);
    context.insert("current_time", &Utc::now().naive_utc());
    context.insert("date", &Utc::today().naive_utc());
    context.insert(
        "staff_fullname",
        &req.session()
            .get::<String>("fullname")
            .unwrap_or_else(|| "Unknown".to_string()),
    );
    context
}

async fn bills(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    context.insert("list_bill_sumary", &dbg!(database::all_bill(true).await?));

    tera.render_response("bill/bills.html", &context)
}
async fn staff(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    context.insert("display", &database::all_staff().await?);
    tera.render_response("staff.html", &context)
}
async fn finance(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let context = base_context(&req);
    tera.render_response("finance.html", &context)
}
async fn customer(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = base_context(&req);
    context.insert("display", &database::all_customer().await?);
    tera.render_response("customer.html", &context)
}
async fn statistic(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let context = base_context(&req);
    tera.render_response("statistic.html", &context)
}
async fn index(req: Request<()>) -> Result<Response> {
    if req.session().get::<String>("user").is_some() {
        Ok(Redirect::new("/manage").into())
    } else {
        let mut tera = TERA.lock().await;
        tera.full_reload()?;
        let context = Context::new();

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
                let name = database::get_staff_name(username.as_str()).await?;
                req.session_mut().insert("fullname", name)?;
                req.session_mut().insert("username", username)?;
                Ok(res)
            } else {
                let mut tera = TERA.lock().await;
                tera.full_reload()?;
                let mut context = Context::new();
                context.insert("msg", "Wrong username or password");
                tera.render_response("index.html", &context)
            }
        }
        Session::Logout { username: _ } => {
            req.session_mut().destroy();
            Ok(Redirect::new("/").into())
        }
    }
}
fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    let mut tide = tide::with_state(());

    tide.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        rand::random::<[u8; 32]>().as_ref(),
    ));
    struct Auth;
    #[async_trait::async_trait]
    impl Middleware<()> for Auth {
        async fn handle(&self, req: Request<()>, next: tide::Next<'_, ()>) -> Result<Response> {
            if req.session().get::<String>("username").is_some() {
                Ok(next.run(req).await)
            } else {
                Ok(Redirect::new("/").into())
            }
        }
    }

    tide.at("/").reset_middleware().get(index);
    tide.at("/login").post(login);
    tide.at("/assert").serve_dir("assert").unwrap();

    tide.at("/new_bill").with(Auth).get(bill::new_bill);
    tide.at("/new_bill/edit_info")
        .with(Auth)
        .get(bill::edit_info);
    tide.at("/new_bill/add_medicine")
        .with(Auth)
        .get(bill::add_medicine);
    tide.at("/new_bill/edit_medicine")
        .with(Auth)
        .get(bill::edit_medicine);
    tide.at("/new_bill/complete").with(Auth).get(bill::complete);

    tide.at("/manage").with(Auth).get(manage::manage_page);
    tide.at("/manage/add_medicine")
        .with(Auth)
        .get(manage::add_medicine);
    tide.at("/manage/edit_medicine")
        .with(Auth)
        .get(manage::edit_medicine);

    tide.at("/staff").with(Auth).get(staff);
    tide.at("/bills").with(Auth).get(bills);
    tide.at("/finance").with(Auth).get(finance);
    tide.at("/customer").with(Auth).get(customer);
    tide.at("/statistic").with(Auth).get(statistic);
    tide.at("/find_drug")
        .with(Auth)
        .get(|req: Request<()>| async move {
            #[derive(Deserialize)]
            struct FindMedicine {
                medicine_name: String,
            }
            let query = req.query::<FindMedicine>()?;
            let res: Response = Response::builder(200)
                .body(serde_json::to_value(
                    database::find_drug(&query.medicine_name, "")
                        .await?
                        .into_iter()
                        .filter(|me| me.medicine_quantity > Some(0))
                        .collect::<Vec<_>>(),
                )?)
                .into();
            Ok(res)
        });
    block_on(database::migrate())?;
    Ok(block_on(tide.listen("0.0.0.0:8080"))?)
}
