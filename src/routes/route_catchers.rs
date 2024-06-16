
pub mod base {
    use rocket::{catch, Request};

    #[catch(500)]
    pub fn internal_error() -> &'static str {
        "Whoops! Looks like we messed up."
    }
    
    #[catch(404)]
    pub fn not_found(req: &Request) -> String {
        format!("I couldn't find '{}'. Try something else?", req.uri())
    }
}

pub mod api {
    use rocket::{catch, Request};

    #[catch(500)]
    pub fn internal_error_api() -> &'static str {
        "Whoops! Looks like we messed up the api call."
    }
    
    
    #[catch(404)]
    pub fn not_found_api(req: &Request) -> String {
        format!("I couldn't find the api endpoint '{}'. Try something else?", req.uri())
    }
}