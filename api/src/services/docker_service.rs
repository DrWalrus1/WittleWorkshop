use crate::models::errors::Error;
use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

const SEPARATOR: &[u8] = b"\r\n\r\n";
pub const GET_IMAGES_JSON: &[u8] = b"GET /images/json HTTP/1.0\r\n\r\n";
pub const GET_CONTAINERS_JSON: &[u8] = b"GET /containers/json HTTP/1.0\r\n\r\n";

async fn execute_docker_request(path: &str, request: &[u8]) -> Result<String, Error> {
    let mut socket = UnixStream::connect(path).unwrap();
    println!("Created the UnixStream");

    socket.write_all(request).unwrap();
    println!("Wrote to stream");
    let mut response = vec![];

    socket.read_to_end(&mut response).unwrap();

    let position = response
        .windows(SEPARATOR.len())
        .position(|window| window == SEPARATOR);
    match position {
        Some(index) => {
            println!("Body Found at index {}", index);
            let body = String::from_utf8(response[index + SEPARATOR.len()..].to_vec())
                .unwrap()
                .trim()
                .to_string();
            Ok(body)
        }
        None => {
            println!("No body found");
            Err(Error {
                code: String::from("500"),
                message: String::from("Internal Server Error"),
                detail: String::from("No body found in response"),
            })
        }
    }
}

pub async fn get_images_linux(path: &str) -> Result<String, Error> {
    execute_docker_request(path, GET_IMAGES_JSON).await
}

pub async fn get_containers_linux(path: &str) -> Result<String, Error> {
    execute_docker_request(path, GET_CONTAINERS_JSON).await
}
