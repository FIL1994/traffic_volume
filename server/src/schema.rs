use juniper::EmptyMutation;
use juniper::FieldResult;
use juniper::RootNode;
use mongodb::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use traffic::TrafficYearData;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrafficData {
    #[serde(rename = "_id")]
    pub id: ObjectId,
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

#[juniper::object]
impl TrafficData {
    fn lhrs(&self) -> i32 {
        self.lhrs
    }

    fn hwy_number(&self) -> i32 {
        self.hwy_number
    }
}

pub struct Context {
    records: Vec<TrafficData>,
}
impl Context {
    pub fn new(records: Vec<TrafficData>) -> Self {
        Context { records: records }
    }
}
impl juniper::Context for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context | &self | {
    field traffic(&executor, id: String) -> FieldResult<TrafficData> {
        let context = executor.context();

        Ok(TrafficData{
            id: ObjectId::new().unwrap(),
            lhrs: 2,
            hwy_number: 1,
            hwy_type: "type".to_string(),
            location_desc: "desc".to_string(),
            reg: "reg".to_string(),
            section_length: 1.0,
            connecting_link_length: 1.0,
            secondary_desc: "second desc".to_string(),
            travel_patterns: HashMap::new()
        })
    },
    field traffics(&executor) -> FieldResult<&Vec<TrafficData>> {
        Ok(&executor.context().records)
    }
});

type MutationRoot = EmptyMutation<Context>;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot::new())
}
