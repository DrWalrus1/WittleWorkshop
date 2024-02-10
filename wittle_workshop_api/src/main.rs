#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};

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
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize, Deserialize)]
enum ContainerCommand {
    Start,
    Stop,
}

impl fmt::Display for ContainerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ContainerCommand::Start => write!(f, "Start"),
            ContainerCommand::Stop => write!(f, "Stop"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ContainerRequest {
    command: ContainerCommand,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContainerResponse {
    result: String,
}

#[post("/container", data = "<container_request>")]
fn hello(container_request: Json<ContainerRequest>) -> Json<ContainerResponse> {
    let result = ContainerResponse {
        result: container_request.command.to_string(),
    };
    return Json::from(result);
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![hello])
}
