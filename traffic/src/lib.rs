use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, GraphQLObject)]
pub struct TrafficYearData {
    pub year: i32,
    /** Design Hour Volume */
    pub dhv: f64,
    pub directional_split: f64,
    /** Annual Average Daily Traffic */
    pub aadt: i32,
    pub aadt_yearly_change: f64,
    pub aadt_10_year_change: Option<f64>,
    /** Summer Average Daily Traffic */
    pub sadt: i32,
    /** Summer Average Weekday Traffic */
    pub sawdt: i32,
    /** Winter Average Daily Traffic */
    pub wadt: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, GraphQLObject)]
pub struct TravelPattern {
    pub pattern: String,
    pub years: Vec<TrafficYearData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficData {
    /** Linear Highway Referencing System */
    pub lhrs: i32,
    pub hwy_number: i32,
    pub hwy_type: String,
    pub location_desc: String,
    /** Regional */
    pub reg: String,
    pub section_length: f64,
    pub connecting_link_length: f64,
    /** (for Connecting Links, Regional Boundarys,etc) */
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
                    years: vec![year],
                });
            }
        }
    }
}
