use rocket::{get, response::content, routes};
use tera::{Context, Tera};
use crate::Config;

pub fn get_html_routes() -> Vec<rocket::Route> {
    routes![
        app_root,
        docker_page,
        render_plan_page,
        test_htmx
    ]
}

#[get("/")]
pub fn app_root(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Tera = &state.templates;
    let context: Context = Context::new();

    let render_result = tera.render("home.html", &context).unwrap();
    return content::RawHtml(render_result);
}

#[get("/managedocker")]
pub fn docker_page(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Tera = &state.templates;
    let context: Context = Context::new();

    let render_result = tera.render("docker.html", &context).unwrap();
    return content::RawHtml(render_result);
}


#[get("/plans")]
pub fn render_plan_page(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Tera = &state.templates;
    let context: Context = Context::new();

    let render_result = tera.render("plans.html", &context).unwrap();
    return content::RawHtml(render_result);
}

#[get("/testhtmx", format = "text/html")]
pub fn test_htmx() -> content::RawHtml<String>{
    content::RawHtml(String::from("<div id='test' hx-get='/testhtmx'>Hello, World!</div>"))
}