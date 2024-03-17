use crate::commands::docker_commands::CommandHandler;
use crate::models::api_bodies::{ContainerRequest, DockerImagesResponse, ApiResponse};
use rocket::serde::json::Json;

#[post("/get", format = "json", data = "<payload>")]
pub async fn get_containers(payload: String) -> ApiResponse<DockerImagesResponse> {
    let parsed_container_request: Result<ContainerRequest, serde_json::Error> = serde_json::from_str(&payload);
    let container_request = match parsed_container_request {
        Ok(container_request) => container_request,
        Err(_) => {     
            let result: DockerImagesResponse = DockerImagesResponse {
                images: Vec::new(),
            };   
            return ApiResponse::BadRequest(Json::from(result))
        },
    };
    let _ = container_request.command.execute().await;
    let result: DockerImagesResponse = DockerImagesResponse {
        images: container_request.command.execute().await.unwrap(),
    };
    ApiResponse::Ok(Json::from(result))
}
