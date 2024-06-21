#[macro_use]
extern crate rocket;
use dockworker::Docker;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{process, thread};
use tera::Tera;
use wittle_workshop_api::routes::*;
use wittle_workshop_api::{routes, Config};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
async fn rocket() -> _ {
    let docker: Docker = match Docker::connect_with_defaults() {
        Ok(docker) => docker,
        Err(e) => {
            println!("Error connecting to Docker: {}", e);
            process::exit(1);
        }
    };

    let templates = match Tera::new("templates/**/*.html") {
        Ok(t) => Arc::new(Mutex::new(t)),
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    setup_template_watcher(&templates);

    let config: Config = Config {
        templates,
        docker,
        // db_pool: pool,
    };

    rocket::build()
        .manage(config)
        .attach(CORS)
        .register(
            "/",
            catchers![
                route_catchers::base::internal_error,
                route_catchers::base::not_found
            ],
        )
        .register(
            "/api",
            catchers![
                route_catchers::api::internal_error_api,
                route_catchers::api::not_found_api
            ],
        )
        .mount("/", html_routes::get_html_routes())
        .mount("/api/docker", docker_routes::get_docker_routes())
        .mount(
            "/api/services",
            routes![routes::service_routes::get_all_services],
        )
        .mount("/public", rocket::fs::FileServer::from("./public/"))
}

fn setup_template_watcher(template_ref: &Arc<Mutex<Tera>>) {
    let template_clone = Arc::clone(&template_ref);
    thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let path = Path::new("./templates/");
        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
        watcher.watch(&path, RecursiveMode::Recursive).unwrap();

        for res in rx {
            match res {
                Ok(_) => {
                    template_clone.lock().unwrap().full_reload().unwrap();
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
