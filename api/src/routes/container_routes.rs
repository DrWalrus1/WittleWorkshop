use crate::commands::docker_commands::CommandHandler;
use crate::models::api_bodies::{ContainerRequest, ContainerResponse};
use rocket::serde::json::Json;

#[derive(Responder)]
pub enum ApiResponse<T> {
    #[response(status = 200, content_type = "json")]
    Ok(Json<T>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<T>)
}


#[post("/container", format = "json", data = "<payload>")]
pub fn container(payload: String) -> ApiResponse<ContainerResponse> {
    let parsed_container_request: Result<ContainerRequest, serde_json::Error> =
        serde_json::from_str(&payload);
    match parsed_container_request {
        Ok(container_request) => {
            dbg!(container_request.command.execute().unwrap().trim());
            let result: ContainerResponse = ContainerResponse {
                result: container_request.command.to_string(),
            };
            ApiResponse::Ok(Json::from(result))
        }
        Err(_) => {     
            let result: ContainerResponse = ContainerResponse {
                result: "Failed to parse request".to_string(),
            };   
            ApiResponse::BadRequest(Json::from(result))
        },
    }
}
