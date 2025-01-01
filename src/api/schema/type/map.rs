use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    /// Name of the map.
    pub name: String,
    /// Skin of the map.
    pub skin: String,
    /// Position X of the map.
    pub x: i32,
    /// Position Y of the map.
    pub y: i32,
    pub content: Option<MapContent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapContent {
    /// Type of the content.
    pub r#type: MapContentType,
    /// Code of the content.
    pub code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MapContentType {
    Monster,
    Resource,
    Workshop,
    Bank,
    #[serde(rename = "grand_exchange")]
    GrandExchange,
    #[serde(rename = "tasks_master")]
    TasksMaster,
    #[serde(rename = "santa_claus")]
    SantaClaus,
}
