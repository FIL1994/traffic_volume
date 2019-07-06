use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, GraphQLObject)]
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

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct TravelPattern {
    pub pattern: String,
    pub years: Vec<TrafficYearData>,
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
    pub travel_patterns: Vec<TravelPattern>,
}

impl TrafficData {
    pub fn add_year(&mut self, key: String, year: TrafficYearData) {
        match self.travel_patterns.iter_mut().find(|t| t.pattern == key) {
            Some(pattern) => {
                pattern.years.push(year);
            }
            _ => {
                self.travel_patterns.push(TravelPattern {
                    pattern: key,
                    years: vec![year]
                });
            }
        }
    }
}
