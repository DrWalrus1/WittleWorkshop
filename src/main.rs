#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::yansi::Paint;
use rocket::{Request, Response};
use wittle_workshop_api::{routes, services, Config};

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
    let db_address = "127.0.0.1";
    let db_port = "5432";
    let db_username = "admin";
    let db_password = "Password1";
    let db_name = "Test";
    let pool = services::connect_db(db_address, db_port, db_username, db_password, db_name).await;

    let config: Config = Config {
        docker_socket_path: if let Ok(path) = std::env::var("DOCKER_SOCKET_PATH") {
            path
        } else {
            println!(
                "{}",
                "WARNING: DOCKER_SOCKET_PATH not set, using default path /var/run/docker.sock"
                    .bold()
                    .yellow()
            );
            String::from("/var/run/docker.sock")
        },
        db_pool: pool,
    };
    rocket::build()
        .manage(config)
        .attach(CORS)
        .mount("/docker", routes![routes::docker_routes::docker_base_get, routes::docker_routes::docker_base_post])
        .mount(
            "/services",
            routes![routes::service_routes::get_all_services],
        )
        // .mount("/", rocket::fs::FileServer::from("../client/dist"))
        .mount("/public", rocket::fs::FileServer::from("./public/"))
        .mount("/tera", routes![routes::tera_test])
}
