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
    let mut traffic_data: Vec<bson::Document> = Vec::new();

    let mut traffic: Vec<TrafficData> = Vec::new();

    let file = File::open("traffic_volumes.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.records() {
        use csv::StringRecord;

        fn get_data(record: &StringRecord, index: usize) -> String {
            record.get(index).unwrap().trim().to_string()
        }

        fn to_i32(string: String) -> i32 {
            string.parse::<i32>().unwrap()
        }

        fn to_f32(string: String) -> Result<f32, String> {
            let mut parseable_string = string.clone();

            match parseable_string.find(".") {
                Some(_v) => (),
                None => parseable_string.push_str(".0"),
            };

            match parseable_string.parse::<f32>() {
                Ok(val) => Ok(val),
                Err(_) => Err(String::from("failed to parse string to f32")),
            }
        }

        match result {
            Ok(record) => {
                use std::panic;

                    let lhrs = to_i32(get_data(&record, 0));

                    match traffic.iter_mut().find(|t| t.lhrs == lhrs) {
                        Some(traffic_record) => {
                            let travel_pattern = get_data(&record, 11);
                            let record_option = &traffic_record.travel_patterns.get(&travel_pattern);
                            match record_option {
                                Some(travel_pattern_data) => {
                                    let years = &travel_pattern_data.years;

                            //         years.push(
                            //             TrafficYearData {
                            //     year: to_i32(get_data(&record, 2)),
                            //     dhv: to_f32(get_data(&record, 12)).unwrap(),
                            //     directional_split: to_f32(get_data(&record, 13)).unwrap(),
                            //     aadt: to_i32(get_data(&record, 14)),
                            //     aadt_yearly_change: to_f32(get_data(&record, 15)).unwrap(),
                            //     aadt_10_year_change: match to_f32(get_data(&record, 16)) {
                            //         Ok(val) => Some(val),
                            //         Err(_) => None,
                            //     },
                            //     sadt: to_i32(get_data(&record, 17)),
                            //     sawdt: to_i32(get_data(&record, 18)),
                            //     wadt: to_i32(get_data(&record, 19)),
                            // }
                            //         );
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            let mut traffic_data = TrafficData {
                                lhrs: lhrs,
                                hwy_number: to_i32(get_data(&record, 3)),
                                hwy_type: get_data(&record, 5),
                                location_desc: get_data(&record, 6),
                                reg: get_data(&record, 7),
                                section_length: to_f32(get_data(&record, 8)).unwrap(),
                                connecting_link_length: to_f32(get_data(&record, 9)).unwrap(),
                                secondary_desc: get_data(&record, 10),
                                travel_patterns: HashMap::new(),
                            };

                            let years: Vec<TrafficYearData> = vec![TrafficYearData {
                                year: to_i32(get_data(&record, 2)),
                                dhv: to_f32(get_data(&record, 12)).unwrap(),
                                directional_split: to_f32(get_data(&record, 13)).unwrap(),
                                aadt: to_i32(get_data(&record, 14)),
                                aadt_yearly_change: to_f32(get_data(&record, 15)).unwrap(),
                                aadt_10_year_change: match to_f32(get_data(&record, 16)) {
                                    Ok(val) => Some(val),
                                    Err(_) => None,
                                },
                                sadt: to_i32(get_data(&record, 17)),
                                sawdt: to_i32(get_data(&record, 18)),
                                wadt: to_i32(get_data(&record, 19)),
                            }];

                            traffic_data
                                .travel_patterns
                                .insert(get_data(&record, 11), TravelPatternData { years: years });
                        }
                    };


                    let traffic_record = TrafficDataRecord {
                        lhrs: to_i32(get_data(&record, 0)),
                        os: to_f32(get_data(&record, 1)).unwrap(),
                        year: to_i32(get_data(&record, 2)),
                        hwy_number: to_i32(get_data(&record, 3)),
                        hwy_type: get_data(&record, 5),
                        location_desc: get_data(&record, 6),
                        reg: get_data(&record, 7),
                        section_length: to_f32(get_data(&record, 8)).unwrap(),
                        connecting_link_length: to_f32(get_data(&record, 9)).unwrap(),
                        secondary_desc: get_data(&record, 10),
                        travel_pattern: get_data(&record, 11),
                        dhv: to_f32(get_data(&record, 12)).unwrap(),
                        directional_split: to_f32(get_data(&record, 13)).unwrap(),
                        aadt: to_i32(get_data(&record, 14)),
                        aadt_yearly_change: to_f32(get_data(&record, 15)).unwrap(),
                        aadt_10_year_change: match to_f32(get_data(&record, 16)) {
                            Ok(val) => Some(val),
                            Err(_) => None,
                        },
                        sadt: to_i32(get_data(&record, 17)),
                        sawdt: to_i32(get_data(&record, 18)),
                        wadt: to_i32(get_data(&record, 19)),
                    };


            }
            Err(_) => {}
        }
    }

    traffic_data

}

fn main() {
    let traffic_data: Vec<bson::Document> = get_traffic_data();
    println!("Found {} records", traffic_data.len());

    let client = Client::connect("localhost", 27017).expect("failed to initialize client");

    let db = client.db("mydb");
    let traffic_col = db.collection("traffic");

    // inserting everything at once will fail
    for chunk in traffic_data.chunks(1000) {
        traffic_col.insert_many(chunk.to_vec(), None).unwrap();
    }
}
