pub mod response;
pub mod r#type;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: u16,
    pub message: String,
}

// Wrapper struct used only for deserialization
#[derive(Clone, Debug, Deserialize)]
pub struct SchemaWrapper<T> {
    pub data: T,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PagedSchemaWrapper<T> {
    pub data: T,
    pub total: u32,
    pub page: u32,
    pub size: u32,
    pub pages: u32,
}
