use serde::Serialize;

pub mod process;

#[derive(Serialize)]
pub struct Item {
    pub description: String,
    pub id: String,
    pub image: String,
    pub instance_of: Vec<String>,
    pub label: String,
    pub wikipedia_title: String,
    pub population: u64,
}
