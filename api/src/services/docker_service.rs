use crate::models::errors::Error;

// TODO: MAKE THIS WORK FOR LINUX!
pub async fn get_images_windows() -> Result<reqwest::Response, Error> {
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