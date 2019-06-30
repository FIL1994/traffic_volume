use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TrafficYearData {
    pub year: i32,
    pub dhv: f32,
    pub directional_split: f32,
    pub aadt: i32,
    pub aadt_yearly_change: f32,
    pub aadt_10_year_change: Option<f32>,
    pub sadt: i32,
    pub sawdt: i32,
    pub wadt: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TravelPatternData {
    pub years: Vec<TrafficYearData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrafficData {
    pub lhrs: i32,
    pub hwy_number: i32,
    pub hwy_type: String,
    pub location_desc: String,
    pub reg: String,
    pub section_length: f32,
    pub connecting_link_length: f32,
    pub secondary_desc: String,
    pub travel_patterns: HashMap<String, TravelPatternData>,
}