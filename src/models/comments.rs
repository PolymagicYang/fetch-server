use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Comments {
    pub id: i32,
    pub body: String,
    pub media_item_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime, 
}