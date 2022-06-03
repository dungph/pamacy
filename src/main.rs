use async_std::{sync::Mutex, task::block_on};
use once_cell::sync::Lazy;
use serde::Serialize;
use tera::{Context, Tera};
use tide::{Request, Response, Result};
use tide_tera::TideTeraExt;

static TERA: Lazy<Mutex<Tera>> = Lazy::new(|| Mutex::new(Tera::new("templates/*.html").unwrap()));

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
fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    let mut server = tide::with_state(());

    // comment
    server.at("/").get(manage_page);
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
