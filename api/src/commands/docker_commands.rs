use crate::models::docker_models::images::Image;
use crate::models::errors::Error;
use rocket::serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::process::Command;

#[async_trait]
pub trait CommandHandler<T> {
    async fn execute(&self) -> Result<T, Error>;
}

#[derive(Serialize, Deserialize)]
pub enum ContainerCommand {
    Start,
    Stop,
}

#[cfg(target_family = "windows")]
#[async_trait]
impl CommandHandler<String> for ContainerCommand {
    async fn execute(&self) -> Result<String, Error> {
        let result = reqwest::get("http://127.0.0.1:2375/images/json").await.expect("Failed to call api").text_with_charset("utf-8").await.expect("Failed to get text");

        let parsed_result: Result<Vec<Image>, serde_json::Error> = serde_json::from_str(&result);
        match parsed_result {
            Ok(_) => {
                println!("IS OK!");
            }
            Err(error) => {
                println!("Something went wrong");
                println!("{}", error);
            }
        }
        Ok(result)
    }
}

impl fmt::Display for ContainerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ContainerCommand::Start => write!(f, "Start"),
            ContainerCommand::Stop => write!(f, "Stop"),
        }
    }
}
