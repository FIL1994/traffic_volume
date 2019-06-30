use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};

use std::collections::HashMap;
use std::fs::File;

use std::num::{ParseFloatError, ParseIntError};

mod custom_error;
use custom_error::CustomError;

mod traffic;
use traffic::{TrafficData, TrafficYearData, TravelPatternData};

fn add_record(
    traffic: &mut Vec<TrafficData>,
    record: csv::StringRecord,
) -> Result<(), CustomError> {
    fn get_data(record: &csv::StringRecord, index: usize) -> String {
        record.get(index).unwrap().trim().to_string()
    }

    fn to_i32(string: String) -> Result<i32, ParseIntError> {
        Ok(string.parse::<i32>())?
    }

    fn to_f32(string: String) -> Result<f32, ParseFloatError> {
        let mut parseable_string = string.clone();

        match parseable_string.find(".") {
            Some(_v) => (),
            None => parseable_string.push_str(".0"),
        };

        Ok(parseable_string.parse::<f32>())?
    }

    let lhrs = to_i32(get_data(&record, 0))?;

    match traffic.iter_mut().find(|t| t.lhrs == lhrs) {
        Some(traffic_record) => {
            let year_record = TrafficYearData {
                year: to_i32(get_data(&record, 2))?,
                dhv: to_f32(get_data(&record, 12))?,
                directional_split: to_f32(get_data(&record, 13))?,
                aadt: to_i32(get_data(&record, 14))?,
                aadt_yearly_change: to_f32(get_data(&record, 15))?,
                aadt_10_year_change: match to_f32(get_data(&record, 16)) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                sadt: to_i32(get_data(&record, 17))?,
                sawdt: to_i32(get_data(&record, 18))?,
                wadt: to_i32(get_data(&record, 19))?,
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
                hwy_number: to_i32(get_data(&record, 3))?,
                hwy_type: get_data(&record, 5),
                location_desc: get_data(&record, 6),
                reg: get_data(&record, 7),
                section_length: to_f32(get_data(&record, 8))?,
                connecting_link_length: to_f32(get_data(&record, 9))?,
                secondary_desc: get_data(&record, 10),
                travel_patterns: HashMap::new(),
            };

            let years: Vec<TrafficYearData> = vec![TrafficYearData {
                year: to_i32(get_data(&record, 2))?,
                dhv: to_f32(get_data(&record, 12))?,
                directional_split: to_f32(get_data(&record, 13))?,
                aadt: to_i32(get_data(&record, 14))?,
                aadt_yearly_change: to_f32(get_data(&record, 15))?,
                aadt_10_year_change: match to_f32(get_data(&record, 16)) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                sadt: to_i32(get_data(&record, 17))?,
                sawdt: to_i32(get_data(&record, 18))?,
                wadt: to_i32(get_data(&record, 19))?,
            }];

            traffic_data
                .travel_patterns
                .insert(get_data(&record, 11), TravelPatternData { years: years });

            traffic.push(traffic_data)
        }
    };

    Ok(())
}

fn get_traffic_data() -> Vec<bson::Document> {
    let mut traffic: Vec<TrafficData> = Vec::new();

    let file = File::open("traffic_volumes.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.records() {
        match result {
            Ok(record) => match add_record(&mut traffic, record) {
                _ => {}
            },
            Err(_) => {}
        }
    }

    traffic
        .iter()
        .filter_map(|record| match bson::to_bson(&record) {
            Ok(serialized_data) => {
                if let bson::Bson::Document(document) = serialized_data {
                    Some(document)
                } else {
                    None
                }
            }
            Err(_) => {
                return None;
            }
        })
        .collect::<Vec<bson::Document>>()
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
