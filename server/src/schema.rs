use crate::db::RECORDS;
use juniper::{EmptyMutation, FieldError, FieldResult, RootNode};
use mongodb::oid::ObjectId;
use serde::{Deserialize, Serialize};
use traffic::TravelPattern;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(GraphQLEnum)]
enum SortField {
    LHRS,
    LocationDesc,
    SecondaryDesc,
    HwyNumber,
}

pub struct Context {}
impl juniper::Context for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context | &self | {
    field traffic(&executor, id: String) -> FieldResult<&TrafficData> {
        let records:&Vec<TrafficData> = &RECORDS;

        match records.iter().find(|t| t.id.to_hex() == id) {
            Some(record) => {
                Ok(record)
            }
            _ => {
                Err(FieldError::new("Record not found", graphql_value!({ "internal_error": "Record not found" })))
            }
        }
    },
    field traffics(&executor, page: Option<i32>, page_size: Option<i32>, sort_by: Option<SortField>) -> FieldResult<Vec<TrafficData>> {
        let records:Vec<TrafficData>  = match sort_by {
            Some(sort_by) => {
                match sort_by {
                    SortField::LocationDesc => {
                        let mut r = RECORDS.clone();
                        r.sort_by_key(|r| r.location_desc.clone());
                        r
                    },
                    SortField::HwyNumber => {
                        let mut r = RECORDS.clone();
                        r.sort_by_key(|r| r.hwy_number.clone());
                        r
                    },
                    SortField::SecondaryDesc => {
                        let mut r = RECORDS.clone();
                        r.sort_by_key(|r| r.secondary_desc.clone());
                        r
                    },
                    SortField::LHRS => {
                        let mut r = RECORDS.clone();
                        r.sort_by_key(|r| r.lhrs.clone());
                        r
                    },
                }
            }
            _ => RECORDS.clone()
        };

        let page = page.unwrap_or(1) as usize;
        let page_size = page_size.unwrap_or(5) as usize;

        let start = (page -1) * page_size;
        let end:usize = std::cmp::min(page * page_size, records.len());

        Ok(records[start..end].to_vec())
    }
});

type MutationRoot = EmptyMutation<Context>;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot::new())
}
