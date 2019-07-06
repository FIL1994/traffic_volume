use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};

use std::fs::File;

mod custom_error;
use custom_error::CustomError;

use traffic::{TrafficData, TrafficYearData};

mod string_to_num;
use string_to_num::{to_f64, to_i32};

fn get_data(record: &csv::StringRecord, index: usize) -> String {
    record.get(index).unwrap().trim().to_string()
}

fn add_record(
    traffic: &mut Vec<TrafficData>,
    record: csv::StringRecord,
) -> Result<(), CustomError> {
    let lhrs = to_i32(get_data(&record, 0))?;

    match traffic.iter_mut().find(|t| t.lhrs == lhrs) {
        Some(traffic_record) => {
            let key = get_data(&record, 11);

            traffic_record.add_year(
                key,
                TrafficYearData {
                    year: to_i32(get_data(&record, 2))?,
                    dhv: to_f64(get_data(&record, 12))?,
                    directional_split: to_f64(get_data(&record, 13))?,
                    aadt: to_i32(get_data(&record, 14))?,
                    aadt_yearly_change: to_f64(get_data(&record, 15))?,
                    aadt_10_year_change: match to_f64(get_data(&record, 16)) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    sadt: to_i32(get_data(&record, 17))?,
                    sawdt: to_i32(get_data(&record, 18))?,
                    wadt: to_i32(get_data(&record, 19))?,
                },
            );
        }
        _ => {
            let mut traffic_data = TrafficData {
                lhrs: lhrs,
                hwy_number: to_i32(get_data(&record, 3))?,
                hwy_type: get_data(&record, 5),
                location_desc: get_data(&record, 6),
                reg: get_data(&record, 7),
                section_length: to_f64(get_data(&record, 8))?,
                connecting_link_length: to_f64(get_data(&record, 9))?,
                secondary_desc: get_data(&record, 10),
                travel_patterns: vec![],
            };

            traffic_data.add_year(
                get_data(&record, 11),
                TrafficYearData {
                    year: to_i32(get_data(&record, 2))?,
                    dhv: to_f64(get_data(&record, 12))?,
                    directional_split: to_f64(get_data(&record, 13))?,
                    aadt: to_i32(get_data(&record, 14))?,
                    aadt_yearly_change: to_f64(get_data(&record, 15))?,
                    aadt_10_year_change: match to_f64(get_data(&record, 16)) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                    sadt: to_i32(get_data(&record, 17))?,
                    sawdt: to_i32(get_data(&record, 18))?,
                    wadt: to_i32(get_data(&record, 19))?,
                },
            );

            traffic.push(traffic_data)
        }
    };

    Ok(())
}

fn get_traffic_data() -> Vec<bson::Document> {
    let mut traffic: Vec<TrafficData> = Vec::new();

    for result in csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(File::open("../traffic_volumes.csv").unwrap_or(
            File::open("../data/example.csv").unwrap()
        ))
        .records()
    {
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
            Err(_) => None,
        })
        .collect::<Vec<bson::Document>>()
}

fn main() {
    let traffic_data: Vec<bson::Document> = get_traffic_data();
    println!("Found {} records", traffic_data.len());

    let client = Client::connect("mongo", 27017).expect("failed to initialize client");

    let db = client.db("mydb");
    let traffic_col = db.collection("traffic");

    println!("inserting records...");
    // inserting everything at once will fail
    for chunk in traffic_data.chunks(1000) {
        traffic_col.insert_many(chunk.to_vec(), None).unwrap();
    }
    println!("finished!");
}
