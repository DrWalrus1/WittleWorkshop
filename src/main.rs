#[macro_use]
extern crate rocket;
use std::process;

use dockworker::Docker;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use tera::Tera;
use wittle_workshop_api::routes::{docker_routes, html_routes};
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
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let config: Config = Config {
        templates: templates,
        docker: docker,
        // db_pool: pool,
    };

    rocket::build()
        .manage(config)
        .attach(CORS)
        .mount("/", html_routes::get_html_routes())
        .mount("/api/docker", docker_routes::get_docker_routes())
        .mount("/api/services", routes![routes::service_routes::get_all_services])
        .mount("/public", rocket::fs::FileServer::from("./public/"))
}
