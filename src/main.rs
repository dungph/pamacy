mod database;
use std::{collections::HashMap, vec};

use async_std::{sync::Mutex, task::block_on};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tide::{Middleware, Redirect, Request, Response, Result};
use tide_tera::TideTeraExt;

static TERA: Lazy<Mutex<Tera>> = Lazy::new(|| Mutex::new(Tera::new("templates/*.html").unwrap()));

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
        name: String,
        medicine_type_query: String,
    }

    #[derive(Deserialize, Debug)]
    struct MedicineAddForm {
        new_medicine_id: i32,
        new_medicine_expire_date: String,
        new_medicine_price: i32,
        new_medicine_name: String,
        new_medicine_type: String,
        new_medicine_quantity: i32,
        new_medicine_location: String,
    }

    #[derive(Serialize, Debug)]
    struct ManageMedicineTemplate {
        medicine_id: String,
        medicine_quantity: String,
        medicine_name: String,
        medicine_type: String,
        medicine_price: String,
        medicine_location: String,
    }
    let display: Vec<ManageMedicineTemplate> = if let Ok(find_form) = req.query::<FindForm>() {
        database::find_drug_match_any(
            Some(find_form.name.clone()),
            Some(find_form.name),
            Some(find_form.medicine_type_query),
        )
        .await?
        .iter()
        .map(|v| ManageMedicineTemplate {
            medicine_id: v.medicine_id.to_string(),
            medicine_quantity: v.medicine_quantity.to_string(),
            medicine_name: v.medicine_name.to_string(),
            medicine_type: v.medicine_type.to_string(),
            medicine_price: v.medicine_price.to_string(),
            medicine_location: v.medicine_location.to_string(),
        })
        .collect()
    } else if let Ok(add_form) = req.query::<MedicineAddForm>() {
        database::add_drug(database::DrugInfo {
            medicine_id: add_form.new_medicine_id,
            medicine_expire_date: add_form.new_medicine_expire_date,
            medicine_price: add_form.new_medicine_price,
            medicine_name: add_form.new_medicine_name,
            medicine_type: add_form.new_medicine_type,
            medicine_quantity: add_form.new_medicine_quantity,
            medicine_location: add_form.new_medicine_location,
            ..Default::default()
        })
        .await?;

        let mut all_drug = database::list_drug().await?;
        all_drug.sort_by(|a, b| a.medicine_id.cmp(&b.medicine_id));
        all_drug
            .iter()
            .map(|v| ManageMedicineTemplate {
                medicine_id: v.medicine_id.to_string(),
                medicine_quantity: v.medicine_quantity.to_string(),
                medicine_name: v.medicine_name.to_string(),
                medicine_type: v.medicine_type.to_string(),
                medicine_price: v.medicine_price.to_string(),
                medicine_location: v.medicine_location.to_string(),
            })
            .collect()
    } else {
        let mut all_drug = database::list_drug().await?;
        all_drug.sort_by(|a, b| a.medicine_id.cmp(&b.medicine_id));
        all_drug
            .iter()
            .map(|v| ManageMedicineTemplate {
                medicine_id: v.medicine_id.to_string(),
                medicine_quantity: v.medicine_quantity.to_string(),
                medicine_name: v.medicine_name.to_string(),
                medicine_type: v.medicine_type.to_string(),
                medicine_price: v.medicine_price.to_string(),
                medicine_location: v.medicine_location.to_string(),
            })
            .collect()
    };

    context.insert("display", &display);
    context.insert("medicine_type_list", &database::list_drug_type().await?);
    context.insert(
        "new_medicine_id",
        &database::next_drug_id().await?.to_string(),
    );
    tera.render_response("manage.html", &context)
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
    let form: Session = req.body_form().await?;
    match form {
        Session::Login { username, password } => {
            if LOGIN_CREDENTAL
                .lock()
                .await
                .get(username.as_str())
                .map(|s| s.as_str())
                .unwrap_or_else(|| "")
                .eq(password.as_str())
            {
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

    Ok(block_on(server.listen("0.0.0.0:8080"))?)
}
