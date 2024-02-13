use rocket::{http::ContentType, response::{self, Responder}, Request, Response};
use serde::{Deserialize, Serialize};

use crate::commands::docker_commands::ContainerCommand;

#[derive(Serialize, Deserialize)]
pub struct ContainerRequest {
    pub command: ContainerCommand,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerResponse {
    pub result: String,
}