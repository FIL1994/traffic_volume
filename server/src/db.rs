use crate::schema::TrafficData;
use mongodb::db::ThreadedDatabase;
use mongodb::{Bson, Client, ThreadedClient};

pub fn run() {
    let client = Client::connect("localhost", 27017).expect("failed to initialize client");

    let db = client.db("mydb");
    let traffic_col = db.collection("traffic");

    let mut cursor = traffic_col.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            let data: TrafficData =
                bson::from_bson(Bson::Document(item)).expect("Expected valid BSON deserialization");
            println!("data {:?}", data);
            return;
        }
    }
}
