pub mod images {
    use dockworker::{image::SummaryImage, Docker};
    use rocket::{get, serde::json::Json};
    use crate::{models::api_bodies::ApiResponse, Config};

    #[get("/images")]
    pub async fn get_images(state: &rocket::State<Config>) -> ApiResponse<Vec<SummaryImage>> {
        let docker: &Docker = &state.docker;
        let images = docker.images(true).await.unwrap();

        ApiResponse::Ok(Json(images))
    }
}

pub mod containers {
    use dockworker::container::ContainerFilters;
    use rocket::{get, serde::json::Json};
    use crate::{models::api_bodies::ApiResponse, Config};


    // ----- CONTAINERS -----
    #[get("/containers")]
    pub async fn get_containers(state: &rocket::State<Config>) -> ApiResponse<Vec<dockworker::container::Container>> {
        let docker = &state.docker;
        let containers = docker.list_containers(None, None, None, ContainerFilters::default()).await.unwrap();
        
        ApiResponse::Ok(Json(containers))
    }
}

pub mod networks {
    use dockworker::Docker;
    use rocket::{get, serde::json::Json};

    use crate::{models::api_bodies::ApiResponse, Config};


    // ----- Networks -----
    #[get("/networks")]
    pub async fn get_networks(state: &rocket::State<Config>) -> ApiResponse<Vec<dockworker::network::Network>> {
        let docker: &Docker = &state.docker;
        let networks = docker.list_networks(Default::default()).await.unwrap();
        
        ApiResponse::Ok(Json(networks))
    }
}
