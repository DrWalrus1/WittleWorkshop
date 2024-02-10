#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

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

#[derive(Serialize, Deserialize)]
struct ContainerResponse {
    result: String,
}

#[post("/container", data = "<container_request>")]
fn hello(container_request: Json<ContainerRequest>) -> Json<ContainerResponse> {
    return Json::from(ContainerResponse {
        result: container_request.command.to_string(),
    });
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
