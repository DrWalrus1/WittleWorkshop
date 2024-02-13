use crate::commands::docker_commands::CommandHandler;
use crate::models::api_bodies::{ContainerRequest, ContainerResponse};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[post("/container", format = "json", data = "<payload>")]
pub fn container(payload: String) -> status::Custom<Json<ContainerResponse>> {
    let parsed_container_request: Result<ContainerRequest, serde_json::Error> =
        serde_json::from_str(&payload);
    match parsed_container_request {
        Ok(container_request) => {
            dbg!(container_request.command.execute().unwrap().trim());
            let result = ContainerResponse {
                result: container_request.command.to_string(),
            };
            return status::Custom(Status::Ok, Json::from(result));
        }
        Err(_) => {
            let result = ContainerResponse {
                result: "Failed to parse request".to_string(),
            };
            return status::Custom(Status::BadRequest, Json::from(result));
        },
    }
}
