use crate::models::errors::Error;


pub trait DockerService: Sync {
    fn get_images(&self) -> impl std::future::Future<Output = Result<reqwest::Response, Error>> + Send;
}

pub struct WindowsDockerService;

impl DockerService for WindowsDockerService {
    // Windows implementation
    async fn get_images(&self) -> Result<reqwest::Response, Error> {
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