use std::fs::File;

struct TrafficData {
    lhrs: u32, // Linear Highway Referencing System
    os: f32,
    year: u32,
    hwy_number: u32,
    hwy_type: String,
    location_desc: String,
    reg: String,
    section_length: f32,
    connecting_link_length: f32,
    secondary_desc: String, // (for Connecting Links, Regional Boundarys,etc)
    travel_pattern: String,
    dhv: f32, // design hour volume
    directional_split: f32,
    aadt: u32,
    aadt_yearly_change: f32,
    aadt_10_year_change: Option<f32>,
    sadt: u32,
    sawdt: u32,
    wadt: u32,
}

fn main() {
    let mut traffic_data: Vec<TrafficData> = Vec::new();

    let file = File::open("traffic_volumes.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.records() {
        use csv::StringRecord;

        fn get_data(record: &StringRecord, index: usize) -> String {
            record.get(index).unwrap().trim().to_string()
        }

        fn to_u32(string: String) -> u32 {
            string.parse::<u32>().unwrap()
        }

        fn to_f32(string: String) -> f32 {
            let mut parseable_string = string.clone();

            match parseable_string.find(".") {
                Some(_v) => (),
                None => parseable_string.push_str(".0"),
            };

            parseable_string.parse::<f32>().unwrap()
        }

        match result {
            Ok(record) => {
                use std::panic;

                let result = panic::catch_unwind(|| {
                    let traffic_record = TrafficData {
                        lhrs: to_u32(get_data(&record, 0)),
                        os: to_f32(get_data(&record, 1)),
                        year: to_u32(get_data(&record, 2)),
                        hwy_number: to_u32(get_data(&record, 3)),
                        hwy_type: get_data(&record, 5),
                        location_desc: get_data(&record, 6),
                        reg: get_data(&record, 7),
                        section_length: to_f32(get_data(&record, 8)),
                        connecting_link_length: to_f32(get_data(&record, 9)),
                        secondary_desc: get_data(&record, 10),
                        travel_pattern: get_data(&record, 11),
                        dhv: to_f32(get_data(&record, 12)),
                        directional_split: to_f32(get_data(&record, 13)),
                        aadt: to_u32(get_data(&record, 14)),
                        aadt_yearly_change: 0.0,
                        aadt_10_year_change: Some(0.0),
                        sadt: 1,
                        sawdt: 1,
                        wadt: 1,
                    };

                    traffic_record
                });

                match result {
                    Ok(traffic_record) => traffic_data.push(traffic_record),
                    _ => {
                        println!("failed: \n {:?}", record,);
                        // println!("{}", get_data(&record, 15));
                    }
                };

            }
            Err(_error) => {}
        }
    }

    println!("done! {}", traffic_data.len());
}
