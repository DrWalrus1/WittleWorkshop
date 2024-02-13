use rocket::serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::process::Command;
use crate::models::errors::Error;

pub trait CommandHandler<T> {
    fn execute(&self) -> Result<T, Error>;
}

#[derive(Serialize, Deserialize)]
pub enum ContainerCommand {
    Start,
    Stop,
}

impl CommandHandler<String> for ContainerCommand {
    fn execute(&self) -> Result<String, Error> {
        let result = match self {
            ContainerCommand::Start => {
                Command::new("cmd")
                    .args(["/C", "echo Start"])
                    .output().unwrap()
            },
            ContainerCommand::Stop => {
                Command::new("cmd")
                    .args(["/C", "echo Stop"])
                    .output().unwrap()
            },
        };

        Ok(String::from_utf8(result.stdout).unwrap())
    }
}

impl fmt::Display for ContainerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        dbg!(self.execute().unwrap());
        match self {
            ContainerCommand::Start => write!(f, "Start"),
            ContainerCommand::Stop => write!(f, "Stop"),
        }
    }
}