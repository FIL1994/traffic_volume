use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct TrafficDataRecord {
    lhrs: i32, // Linear Highway Referencing System
    os: f32,
    year: i32,
    hwy_number: i32,
    hwy_type: String,
    location_desc: String,
    reg: String,
    section_length: f32,
    connecting_link_length: f32,
    secondary_desc: String, // (for Connecting Links, Regional Boundarys,etc)
    travel_pattern: String,
    dhv: f32, // design hour volume
    directional_split: f32,
    aadt: i32,
    aadt_yearly_change: f32,
    aadt_10_year_change: Option<f32>,
    sadt: i32,
    sawdt: i32,
    wadt: i32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
struct TrafficYearData {
    year: i32,
    dhv: f32,
    directional_split: f32,
    aadt: i32,
    aadt_yearly_change: f32,
    aadt_10_year_change: Option<f32>,
    sadt: i32,
    sawdt: i32,
    wadt: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TravelPatternData {
    years: Vec<TrafficYearData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrafficData {
    lhrs: i32,
    hwy_number: i32,
    hwy_type: String,
    location_desc: String,
    reg: String,
    section_length: f32,
    connecting_link_length: f32,
    secondary_desc: String,
    travel_patterns: HashMap<String, TravelPatternData>,
}

fn get_traffic_data() -> Vec<bson::Document> {
    let mut traffic_data_records: Vec<bson::Document> = Vec::new();

    let mut traffic: Vec<TrafficData> = Vec::new();

    let file = File::open("test.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.records() {
        use csv::StringRecord;

        fn get_data(record: &StringRecord, index: usize) -> String {
            record.get(index).unwrap().trim().to_string()
        }

        fn to_i32(string: String) -> Result<i32, std::num::ParseIntError> {
            Ok(string.parse::<i32>())?
        }

        fn to_f32(string: String) -> Result<f32, std::num::ParseFloatError> {
            let mut parseable_string = string.clone();

            match parseable_string.find(".") {
                Some(_v) => (),
                None => parseable_string.push_str(".0"),
            };

            Ok(parseable_string.parse::<f32>())?
        }

        match result {
            Ok(record) => {
                let lhrs = to_i32(get_data(&record, 0)).unwrap();

                match traffic.iter_mut().find(|t| t.lhrs == lhrs) {
                    Some(traffic_record) => {
                        let year_record = TrafficYearData {
                            year: to_i32(get_data(&record, 2)).unwrap(),
                            dhv: to_f32(get_data(&record, 12)).unwrap(),
                            directional_split: to_f32(get_data(&record, 13)).unwrap(),
                            aadt: to_i32(get_data(&record, 14)).unwrap(),
                            aadt_yearly_change: to_f32(get_data(&record, 15)).unwrap(),
                            aadt_10_year_change: match to_f32(get_data(&record, 16)) {
                                Ok(val) => Some(val),
                                Err(_) => None,
                            },
                            sadt: to_i32(get_data(&record, 17)).unwrap(),
                            sawdt: to_i32(get_data(&record, 18)).unwrap(),
                            wadt: to_i32(get_data(&record, 19)).unwrap(),
                        };

                        let travel_pattern_key = get_data(&record, 11);
                        match traffic_record.travel_patterns.get_mut(&travel_pattern_key) {
                            Some(travel_pattern_data) => {
                                travel_pattern_data.years.push(year_record);
                            }
                            _ => {
                                traffic_record.travel_patterns.insert(
                                    travel_pattern_key,
                                    TravelPatternData {
                                        years: vec![year_record],
                                    },
                                );
                            }
                        }
                    }
                    _ => {
                        let mut traffic_data = TrafficData {
                            lhrs: lhrs,
                            hwy_number: to_i32(get_data(&record, 3)).unwrap(),
                            hwy_type: get_data(&record, 5),
                            location_desc: get_data(&record, 6),
                            reg: get_data(&record, 7),
                            section_length: to_f32(get_data(&record, 8)).unwrap(),
                            connecting_link_length: to_f32(get_data(&record, 9)).unwrap(),
                            secondary_desc: get_data(&record, 10),
                            travel_patterns: HashMap::new(),
                        };

                        let years: Vec<TrafficYearData> = vec![TrafficYearData {
                            year: to_i32(get_data(&record, 2)).unwrap(),
                            dhv: to_f32(get_data(&record, 12)).unwrap(),
                            directional_split: to_f32(get_data(&record, 13)).unwrap(),
                            aadt: to_i32(get_data(&record, 14)).unwrap(),
                            aadt_yearly_change: to_f32(get_data(&record, 15)).unwrap(),
                            aadt_10_year_change: match to_f32(get_data(&record, 16)) {
                                Ok(val) => Some(val),
                                Err(_) => None,
                            },
                            sadt: to_i32(get_data(&record, 17)).unwrap(),
                            sawdt: to_i32(get_data(&record, 18)).unwrap(),
                            wadt: to_i32(get_data(&record, 19)).unwrap(),
                        }];

                        traffic_data
                            .travel_patterns
                            .insert(get_data(&record, 11), TravelPatternData { years: years });

                        traffic.push(traffic_data)
                    }
                };

            }
            Err(_) => {}
        }
    }

    println!("len {}", traffic.len());

    traffic_data_records
}

fn main() {
    let traffic_data: Vec<bson::Document> = get_traffic_data();
    println!("Found {} records", traffic_data.len());

    // let client = Client::connect("localhost", 27017).expect("failed to initialize client");

    // let db = client.db("mydb");
    // let traffic_col = db.collection("traffic");

    // // inserting everything at once will fail
    // for chunk in traffic_data.chunks(1000) {
    //     traffic_col.insert_many(chunk.to_vec(), None).unwrap();
    // }
}
