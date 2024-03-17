use crate::models::docker_models::images::Image;
use crate::models::errors::Error;
use rocket::serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::future::Future;


#[derive(Serialize, Deserialize)]
pub enum DockerImagesCommand {
    Start,
    Stop,
}


pub struct DockerService;

impl DockerService {
    // Windows implementation
    async fn get_images() -> Result<reqwest::Response, Error> {
        match reqwest::get("http://127.0.0.1:2375/images/json").await {
            Err(_) => {
                return Err(Error {
                    code: String::from("ERROR"),
                    message: String::from("This is the message"),
                    detail: String::from("This is the detail"),
                })
            }
            Ok(response) => return Ok(response)
        };
    }
}

pub trait CommandHandler<T> {
    fn execute(&self) -> impl Future<Output = Result<T, Error>> + Send;
}

impl CommandHandler<Vec<Image>> for DockerImagesCommand {
    async fn execute(&self) -> Result<Vec<Image>, Error> {
        let response = match DockerService::get_images().await {
            Ok(response) => response,
            Err(error) => return Err(error),
        };
        let response_text = match response.text().await {
            Err(_) => {
                return Err(Error {
                    code: String::from("ERROR"),
                    message: String::from("Failed to read response text"),
                    detail: String::from("Failed to read response text"),
                })
            }
            Ok(response_text) => response_text,
        };

        let parsed_result: Vec<Image> = match serde_json::from_str(&response_text) {
            Ok(parsed_result) => parsed_result,
            Err(error) => {
                eprintln!("Something went wrong");
                eprintln!("{}", error);
                return Err(Error {
                    code: String::from("ERROR"),
                    message: String::from("Failed to parse JSON"),
                    detail: String::from("Failed to parse JSON"),
                })
            }
        };
        Ok(parsed_result)
    }
}

impl fmt::Display for DockerImagesCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DockerImagesCommand::Start => write!(f, "Start"),
            DockerImagesCommand::Stop => write!(f, "Stop"),
        }
    }
}
