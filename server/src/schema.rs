use juniper::EmptyMutation;
use juniper::FieldResult;
use juniper::RootNode;
use mongodb::oid::ObjectId;
use serde::{Deserialize, Serialize};
use traffic::TravelPattern;

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
    pub travel_patterns: Vec<TravelPattern>,
}

#[juniper::object]
impl TrafficData {
    fn id(&self) -> String {
        self.id.to_hex()
    }
    fn lhrs(&self) -> i32 {
        self.lhrs
    }
    fn hwy_number(&self) -> i32 {
        self.hwy_number
    }
    fn hwy_type(&self) -> &String {
        &self.hwy_type
    }
    fn location_desc(&self) -> &String {
        &self.location_desc
    }
    fn reg(&self) -> &String {
        &self.reg
    }
    fn section_length(&self) -> f64 {
        self.section_length
    }
    fn connecting_link_length(&self) -> f64 {
        self.connecting_link_length
    }
    fn secondary_desc(&self) -> &String {
        &self.secondary_desc
    }
    fn travel_patterns(&self) -> &Vec<TravelPattern> {
        &self.travel_patterns
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
            travel_patterns: vec![]
        })
    },
    field traffics(&executor, page: Option<i32>, page_size: Option<i32>) -> FieldResult<&[TrafficData]> {
        let records:&Vec<TrafficData> = &executor.context().records;

        let page = page.unwrap_or(1) as usize;
        let page_size = page_size.unwrap_or(5) as usize;

        let start = (page -1) * page_size;
        let end:usize = std::cmp::min(page * page_size, records.len());

        Ok(&records[start..end])
    }
});

type MutationRoot = EmptyMutation<Context>;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot::new())
}