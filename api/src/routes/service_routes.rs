use rocket::serde::json::Json;
use crate::models::api_bodies::{ApiResponse, ServiceResponse};



#[get("/")]
pub async fn get_all_services() -> ApiResponse<ServiceResponse> {
    ApiResponse::Ok(Json::from(ServiceResponse {
        result: String::from("Hello")
    }))
}