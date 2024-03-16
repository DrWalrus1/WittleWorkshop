use crate::models::docker_models::images::Image;
use crate::models::errors::Error;
use rocket::serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::process::Command;

pub trait CommandHandler<T> {
    fn execute(&self) -> Result<T, Error>;
}

#[derive(Serialize, Deserialize)]
pub enum ContainerCommand {
    Start,
    Stop,
}

#[cfg(target_family = "windows")]
impl CommandHandler<String> for ContainerCommand {
    fn execute(&self) -> Result<String, Error> {
        let result = Command::new("cmd")
            .args([
                "/C",
                "curl",
                "-X",
                "GET",
                "http://127.0.0.1:2375/images/json",
            ])
            .output()
            .unwrap();
        let parsed_result: Result<Vec<Image>, serde_json::Error> =
            serde_json::from_str(&String::from_utf8(result.stdout.clone()).unwrap());
        match parsed_result {
            Ok(list_of_images) => {
                println!("IS OK!");
                dbg!(list_of_images);
            },
            Err(error) => {
                println!("Something went wrong bro");
                println!("{}", error);
            }
        }
        Ok(String::from_utf8(result.stdout).unwrap().trim().to_string())
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
