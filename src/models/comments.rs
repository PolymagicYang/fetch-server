use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Comments {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub body: String,
}
