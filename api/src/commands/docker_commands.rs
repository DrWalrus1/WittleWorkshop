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

macro_rules! any_os_command {
    ($command:expr) => {
        #[cfg(target_family = "unix")]
        {

        };
        #[cfg(target_family = "windows")]
    };
}

impl CommandHandler<String> for ContainerCommand {
    fn execute(&self) -> Result<String, Error> {
        let command = format!("echo {}", self.to_string());
        let result =
                Command::new("cmd")
                    .args(["/C", &command])
                    .output().unwrap();
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