use rocket::serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::Error;

pub trait Command<T> {
    fn execute(&self) -> Result<T, Error>;
}

#[derive(Serialize, Deserialize)]
pub enum ContainerCommand {
    Start,
    Stop,
}

impl fmt::Display for ContainerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ContainerCommand::Start => write!(f, "Start"),
            ContainerCommand::Stop => write!(f, "Stop"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ContainerRequest {
    pub command: ContainerCommand,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerResponse {
    pub result: String,
}