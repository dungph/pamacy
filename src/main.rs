use std::collections::HashMap;

use async_std::{sync::Mutex, task::block_on};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tide::{
    http::{Cookie, Url},
    utils::Before,
    Redirect, Request, Response, Result,
};
use tide_tera::TideTeraExt;

static TERA: Lazy<Mutex<Tera>> = Lazy::new(|| Mutex::new(Tera::new("templates/*.html").unwrap()));

#[derive(Clone)]
struct Username(String);

static COOKIES: Lazy<Mutex<HashMap<String, Username>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static LOGIN_CREDENTAL: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::from_iter(
        [("admin".to_owned(), "admin".to_owned())].into_iter(),
    ))
});

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
async fn manage_page(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = Context::new();

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

    context.insert("danhsach", &[&val; 20]);
    tera.render_response("manage.html", &context)
}

async fn new_bill(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;

    let mut context = Context::new();

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

    context.insert("danhsach", &[&val; 20]);
    context.insert("bill_id", "123");
    tera.render_response("new_bill.html", &context)
}

async fn staff(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = Context::new();
    tera.render_response("staff.html", &context)
}
async fn bills(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = Context::new();
    tera.render_response("bills.html", &context)
}
async fn finance(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = Context::new();
    tera.render_response("finance.html", &context)
}
async fn customer(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = Context::new();
    tera.render_response("customer_info.html", &context)
}
async fn statistic(req: Request<()>) -> Result<Response> {
    let mut tera = TERA.lock().await;
    tera.full_reload()?;
    let mut context = Context::new();
    tera.render_response("statistic.html", &context)
}
async fn index(req: Request<()>) -> Result<Response> {
    if req.ext::<Username>().is_some() {
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
                let random_string = base64::encode(rand::random::<[u8; 32]>());
                COOKIES
                    .lock()
                    .await
                    .insert(random_string.clone(), Username(username));
                let mut res: Response = Redirect::new("/manage").into();
                res.insert_cookie(Cookie::new("login", random_string));
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
            if let Some(cookie) = req.cookie("login") {
                COOKIES.lock().await.remove(cookie.value());
            }
            Ok(Redirect::new("/").into())
        }
        _ => Ok(Response::builder(404).build()),
    }
}
fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    let mut server = tide::with_state(());

    server.with(Before(|mut request: Request<()>| async {
        if let Some(cookie) = request.cookie("login") {
            if let Some(username) = COOKIES.lock().await.get(cookie.value()).cloned() {
                request.set_ext(username);
            }
        }
        request
    }));
    // comment
    server.at("/").get(index);
    server.at("/login").post(login);
    server.at("/assert").serve_dir("assert").unwrap();

    server.at("/new_bill").get(new_bill);
    server.at("/manage").get(manage_page);
    server.at("/staff").get(staff);
    server.at("/bills").get(bills);
    server.at("/finance").get(finance);
    server.at("/customer").get(customer);
    server.at("/statistic").get(statistic);

    Ok(block_on(server.listen("0.0.0.0:8080"))?)
}
