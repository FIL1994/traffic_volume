use crate::db::RECORDS;
use juniper::meta::MetaType;
use juniper::{
    Arguments, DefaultScalarValue, EmptyMutation, ExecutionResult, Executor, FieldError,
    FieldResult, GraphQLType, Registry, RootNode,
};
use mongodb::oid::ObjectId;
use serde::{Deserialize, Serialize};
use traffic::{TrafficYearData, TravelPattern};

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

impl TrafficData {
    fn flatten_years(&self) -> Vec<TrafficYearData> {
        self.travel_patterns
            .clone()
            .iter()
            .flat_map(|tp| tp.to_owned().years)
            .collect()
    }

    fn get_avg_aadt(&self) -> i32 {
        let years: Vec<TrafficYearData> = self.flatten_years();
        years.iter().map(|t| t.to_owned().aadt).sum()
    }
}

impl TrafficData {
    fn id(&self) -> String {
        self.id.to_hex()
    }
    fn avg_aadt(&self) -> i32 {
        self.get_avg_aadt().clone().to_owned()
    }
}

impl GraphQLType for TrafficData {
    type Context = AppContext;
    type TypeInfo = ();

    fn name(_: &()) -> Option<&'static str> {
        Some("TrafficData")
    }

    fn meta<'r>(_: &(), registry: &mut Registry<'r>) -> MetaType<'r>
    where
        DefaultScalarValue: 'r,
    {
        let fields = &[
            registry.field::<&String>("id", &()),
            registry.field::<&i32>("lhrs", &()),
            registry.field::<&i32>("hwyNumber", &()),
            registry.field::<&String>("hwyType", &()),
            registry.field::<&String>("locationDesc", &()),
            registry.field::<&String>("reg", &()),
            registry.field::<&f64>("sectionLength", &()),
            registry.field::<&f64>("connectingLinkLength", &()),
            registry.field::<&String>("secondaryDesc", &()),
            registry.field::<Vec<TravelPattern>>("travelPatterns", &()),
            registry.field::<&i32>("avgAadt", &()),
        ];

        registry
            .build_object_type::<TrafficData>(&(), fields)
            .into_meta()
    }

    fn resolve_field(
        &self,
        info: &(),
        field_name: &str,
        _args: &Arguments,
        executor: &Executor<AppContext>,
    ) -> ExecutionResult {
        match field_name {
            "id" => executor.resolve_with_ctx(info, &self.id()),
            "lhrs" => executor.resolve_with_ctx(info, &self.lhrs),
            "hwyNumber" => executor.resolve_with_ctx(info, &self.hwy_number),
            "hwyType" => executor.resolve_with_ctx(info, &self.hwy_type),
            "locationDesc" => executor.resolve_with_ctx(info, &self.location_desc),
            "reg" => executor.resolve_with_ctx(info, &self.reg),
            "sectionLength" => executor.resolve_with_ctx(info, &self.section_length),
            "connectingLinkLength" => executor.resolve_with_ctx(info, &self.connecting_link_length),
            "secondaryDesc" => executor.resolve_with_ctx(info, &self.secondary_desc),
            "travelPatterns" => executor.resolve_with_ctx(info, &self.travel_patterns),
            "avgAadt" => executor.resolve_with_ctx(info, &self.avg_aadt()),
            _ => panic!("Field {} not found on type TrafficData", field_name),
        }
    }
}

#[derive(GraphQLEnum)]
enum SortField {
    LHRS,
    LocationDesc,
    SecondaryDesc,
    HwyNumber,
    HwyType,
    AvgAadt,
}

pub struct AppContext {}
impl juniper::Context for AppContext {}

pub struct QueryRoot;

graphql_object!(QueryRoot: AppContext | &self | {
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
    field traffics(
        &executor,
        page: Option<i32>,
        page_size: Option<i32>,
        sort_by: Option<SortField>,
        sort_asc: Option<bool>
    ) -> FieldResult<Vec<TrafficData>> 
    {
        fn sort<K, F>(f: F) -> Vec<TrafficData> 
            where F: FnMut(&TrafficData) -> K, K: Ord 
        {
            let mut records = RECORDS.clone();
            records.sort_by_key(f);
            records
        }

        let mut records:Vec<TrafficData> = match sort_by {
            Some(sort_by) => {
                match sort_by {
                    SortField::LocationDesc => sort(|r| r.location_desc.clone()),
                    SortField::HwyNumber => sort(|r| r.hwy_number.clone()),
                    SortField::SecondaryDesc => sort(|r| r.secondary_desc.clone()),
                    SortField::LHRS => sort(|r| r.lhrs.clone()),
                    SortField::AvgAadt => sort(|r| r.get_avg_aadt().clone()),
                    SortField::HwyType => sort(|r| r.hwy_type.clone()),
                }
            }
            _ => RECORDS.clone()
        };

        if !sort_asc.unwrap_or(true) { records.reverse(); }

        let page = page.unwrap_or(1) as usize;
        let page_size = page_size.unwrap_or(5) as usize;

        let start = (page -1) * page_size;
        let end: usize = std::cmp::min(page * page_size, records.len());

        Ok(records[start..end].to_vec())
    }
});

type MutationRoot = EmptyMutation<AppContext>;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot::new())
}
