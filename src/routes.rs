use rocket::{get, response::content};
use tera::{Context, Tera};

use crate::Config;

pub mod docker_routes;
pub mod service_routes;


#[get("/")]
pub fn app_root(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Tera = &state.templates;
    let context: Context = Context::new();

    let render_result = tera.render("home.html", &context).unwrap();
    return content::RawHtml(render_result);
}


#[get("/plans")]
pub fn render_plan_page(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Tera = &state.templates;
    let context: Context = Context::new();

    let render_result = tera.render("plans.html", &context).unwrap();
    return content::RawHtml(render_result);
}
