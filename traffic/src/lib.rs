use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use juniper;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TrafficYearData {
    pub year: i32,
    pub dhv: f64,
    pub directional_split: f64,
    pub aadt: i32,
    pub aadt_yearly_change: f64,
    pub aadt_10_year_change: Option<f64>,
    pub sadt: i32,
    pub sawdt: i32,
    pub wadt: i32,
}

#[juniper::object]
impl TrafficYearData {
    fn year(&self) -> i32 {
        self.year
    }

    fn aadt(&self) -> i32 {
        self.aadt
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrafficData {
    pub lhrs: i32,
    pub hwy_number: i32,
    pub hwy_type: String,
    pub location_desc: String,
    pub reg: String,
    pub section_length: f64,
    pub connecting_link_length: f64,
    pub secondary_desc: String,
    pub travel_patterns: HashMap<String, Vec<TrafficYearData>>,
}

impl TrafficData {
    pub fn add_year(&mut self, key: String, year: TrafficYearData) {
        match self.travel_patterns.get_mut(&key) {
            Some(travel_pattern_data) => {
                travel_pattern_data.push(year);
            }
            _ => {
                self.travel_patterns.insert(key, vec![year]);
            }
        }
    }
}
