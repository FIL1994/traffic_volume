use std::fs::File;

struct TrafficData {
    lhrs: u32, // 	Linear Highway Referencing System
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
    println!("Hello, world!");

    let t = TrafficData {
        lhrs: 1,
        os: 0.0,
        year: 2000,
        hwy_number: 1,
        hwy_type: "".to_string(),
        location_desc: "".to_string(),
        reg: "".to_string(),
        section_length: 0.0,
        connecting_link_length: 0.0,
        secondary_desc: "".to_string(),
        travel_pattern: "".to_string(),
        dhv: 0.0,
        directional_split: 0.0,
        aadt: 1,
        aadt_yearly_change: 0.0,
        aadt_10_year_change: Some(0.0),
        sadt: 1,
        sawdt: 1,
        wadt: 1,
    };

    let mut traffic_data: Vec<TrafficData> = Vec::new();

    let file = File::open("traffic_volumes.csv").unwrap();
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.records() {
        use csv::StringRecord;

        fn get_data(record: &StringRecord, index: usize) -> String {
            record.get(index).unwrap().to_string()
        }

        fn to_u32(string: String) -> u32 {
            string.parse::<u32>().unwrap()
        }

        fn to_f32(string: String) -> f32 {
            let mut parseable_string = string.clone();

            match parseable_string.find(".") {
                Some(_v) => {}
                None => {

                    parseable_string.push_str(".0");
                    println!("not found {}", parseable_string);
                }
            };

            parseable_string.parse::<f32>().unwrap()
        }

        match result {
            Ok(record) => {
                use std::panic;
                // let d = get_data(record, 1);

                let result = panic::catch_unwind(|| {
                    let traffic_record = TrafficData {
                        lhrs: to_u32(get_data(&record, 0)),
                        os: to_f32(get_data(&record, 1)),
                        year: to_u32(get_data(&record, 2)),
                        hwy_number: to_u32(get_data(&record, 3)),
                        hwy_type: get_data(&record, 4),
                        location_desc: get_data(&record, 5),
                        reg: get_data(&record, 6),
                        section_length: to_f32(get_data(&record, 7)),
                        connecting_link_length: 0.0,
                        secondary_desc: "".to_string(),
                        travel_pattern: "".to_string(),
                        dhv: 0.0,
                        directional_split: 0.0,
                        aadt: 1,
                        aadt_yearly_change: 0.0,
                        aadt_10_year_change: Some(0.0),
                        sadt: 1,
                        sawdt: 1,
                        wadt: 1,
                    };
                });

                match result {
                    Ok(_r) => {}
                    _ => println!("error"),
                };

            }
            Err(_error) => {}
        }
    }

    println!("done!");
}
