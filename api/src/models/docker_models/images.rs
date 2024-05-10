use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub containers: Option<i32>,
    pub created: Option<u32>,
    pub id: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub parent_id: Option<String>,
    pub repo_digests: Option<Vec<String>>,
    pub repo_tags: Option<Vec<String>>,
    pub shared_size: Option<i32>,
    pub size: Option<usize>,
}
