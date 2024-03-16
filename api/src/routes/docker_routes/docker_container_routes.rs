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


#[post("/get", format = "json", data = "<payload>")]
pub async fn get_containers(payload: String) -> ApiResponse<ContainerResponse> {
    let parsed_container_request: Result<ContainerRequest, serde_json::Error> =
        serde_json::from_str(&payload);
    match parsed_container_request {
        Ok(container_request) => {
            let _ = container_request.command.execute().await;
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

#[post("/somethingElse", format = "json", data = "<payload>")]
pub async fn do_something_else(payload: String) -> ApiResponse<ContainerResponse> {
    let parsed_container_request: Result<ContainerRequest, serde_json::Error> =
        serde_json::from_str(&payload);
    match parsed_container_request {
        Ok(container_request) => {
            dbg!(container_request.command.execute().await.unwrap().trim());
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
