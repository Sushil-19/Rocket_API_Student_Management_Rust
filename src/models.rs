use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub age: u8,
    pub department: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateStudent {
    pub name: String,
    pub age: u8,
    pub department: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudent {
    pub name: String,
    pub age: u8,
    pub department: String,
}
