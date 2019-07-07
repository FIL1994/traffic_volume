use crate::schema::TrafficData;
use mongodb::db::ThreadedDatabase;
use mongodb::{Bson, Client, ThreadedClient};

lazy_static! {
    pub static ref RECORDS: Vec<TrafficData> = { collect_data() };
}

fn collect_data() -> Vec<TrafficData> {
    let client = Client::connect("mongo", 27017).expect("failed to initialize client");

    let db = client.db("mydb");
    let traffic_col = db.collection("traffic");

    let mut data: Vec<TrafficData> = Vec::new();
    let cursor = traffic_col.find(None, None).unwrap();

    for result in cursor {
        if let Ok(item) = result {
            let traffic_item: TrafficData =
                bson::from_bson(Bson::Document(item)).expect("Expected valid BSON deserialization");

            data.push(traffic_item);
        }
    }

    data
}
