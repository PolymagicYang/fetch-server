use bson::{oid::ObjectId};
use serde::{Serialize, Deserialize};
use crate::models::comments::Comments;
use uuid::Uuid;
use tokio;
use mongodb::bson::doc;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
enum MediaType {
	Video(String),
	Audio(String),
	Other(String),
}

#[derive(Debug, Deserialize, Serialize)]
enum LocationType {
	S3(String),
	Local(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	id: Option<ObjectId>,
	name: String,
	note: String,
	media_type: MediaType,
	location: LocationType,
	size: usize,
	published: bool,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	created_at: DateTime<Utc>, 
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	updated_at: DateTime<Utc>,
	device_id: Uuid,
	comments: Vec<Comments>,
}

#[tokio::test]
async fn test_model_correction() {
	use crate::connect_to_mongo;
	let client = connect_to_mongo().await.unwrap();

	let database = client.database("testdb");
	let collection = database.collection::<Media>("meida");
	let new_uuid = uuid::Builder::nil().build();
	let local = chrono::Utc::now();

	let one = Media {
			id: None,
			name: "test".to_string(),
			note: "test".to_string(),
			media_type: MediaType::Video("test".to_string()),
			location: LocationType::S3("test".to_string()),
			size: 1,
			published: true,
			created_at: local, 
			updated_at: local,
			device_id: new_uuid, 
			comments: vec![Comments {id: None, body: "Test".to_string()}],
		};

	collection.insert_one(one, None).await.unwrap();

	let a = collection.find(doc! { "name": "test" }, None).await.unwrap();

    println!("title: {:?}", a);
}

