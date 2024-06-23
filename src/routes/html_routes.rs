use std::sync::{Arc, Mutex};

use crate::{models::api_bodies::ApiResponse, Config};
use dockworker::{container::ContainerFilters, Docker};
use rocket::{get, response::content, routes, serde::json::Json};
use tera::{Context, Tera};

pub fn get_html_routes() -> Vec<rocket::Route> {
    routes![
        app_root,
        docker_page,
        render_plan_page,
        test_htmx,
        test_htmx_json,
        get_docker_container_html
    ]
}

#[get("/")]
pub fn app_root(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera_arc: &Arc<Mutex<Tera>> = &state.templates;
    let tera = tera_arc.lock().unwrap();
    let context: Context = Context::new();

    let render_result = tera.render("home.html", &context).unwrap();
    return content::RawHtml(render_result);
}

#[get("/managedocker")]
pub fn docker_page(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Arc<Mutex<Tera>> = &state.templates;
    let tera = tera.lock().unwrap();
    let context: Context = Context::new();

    let render_result = tera.render("docker.html", &context).unwrap();
    return content::RawHtml(render_result);
}

#[get("/plans")]
pub fn render_plan_page(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Arc<Mutex<Tera>> = &state.templates;
    let tera = tera.lock().unwrap();
    let context: Context = Context::new();

    let render_result = tera.render("plans.html", &context).unwrap();
    return content::RawHtml(render_result);
}

#[get("/testhtmx", format = "html")]
pub fn test_htmx() -> content::RawHtml<String> {
    content::RawHtml(String::from("Hello, World!"))
}

#[get("/getdockercontainer", format = "html")]
pub async fn get_docker_container_html(state: &rocket::State<Config>) -> content::RawHtml<String> {
    let tera: &Arc<Mutex<Tera>> = &state.templates;
    let docker: &Docker = &state.docker;
    let filter = ContainerFilters::new();
    let containers = docker
        .list_containers(None, None, None, filter)
        .await
        .unwrap();

    let first_container = &containers[0];
    let container_name = &first_container.Names[0];
    let container_status = &first_container.Status;
    // setup context

    let mut context: Context = Context::new();
    context.insert("container_name", container_name);
    context.insert("container_status", container_status);
    let tera = tera.lock().unwrap();
    let render_result = tera
        .render("components/DockerContainerView.html", &context)
        .unwrap();
    return content::RawHtml(render_result);
}

// Probably should keep this functionality in a different file, but for now it's here.
// future idea would be to enable people to a custom front-end if wanted.
#[get("/testhtmx", rank = 2, format = "json")]
pub fn test_htmx_json() -> ApiResponse<String> {
    ApiResponse::Ok(Json(String::from("Hello, World!")))
}
