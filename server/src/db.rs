use mongodb::db::ThreadedDatabase;
use mongodb::oid::ObjectId;
use mongodb::{Bson, Client, ThreadedClient};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrafficData {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub lhrs: i32,
    pub hwy_number: i32,
    pub hwy_type: String,
    pub location_desc: String,
    pub reg: String,
    pub section_length: f32,
    pub connecting_link_length: f32,
    pub secondary_desc: String,
}

pub fn run() {
    let client = Client::connect("localhost", 27017).expect("failed to initialize client");

    let db = client.db("mydb");
    let traffic_col = db.collection("traffic");

    let mut cursor = traffic_col.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            let data: TrafficData =
                bson::from_bson(Bson::Document(item)).expect("Expected valid BSON deserialization");
            println!("d {:?}", data);
            return;
        }
    }
}
