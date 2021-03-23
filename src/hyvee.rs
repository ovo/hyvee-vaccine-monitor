use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Variables {
    radius: f32,
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub search_pharmacies_near_point: Vec<SearchPharmaciesNearPoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchPharmaciesNearPoint {
    pub distance: f64,
    pub location: Location,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub location_id: String,
    pub name: String,
    pub nickname: Option<String>,
    pub phone_number: Option<String>,
    pub business_code: String,
    pub is_covid_vaccine_available: bool,
    pub covid_vaccine_eligibility_terms: String,
    pub address: ResponseAddress,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAddress {
    pub line1: String,
    pub line2: serde_json::Value,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub tracing: Tracing,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tracing {
    pub version: i64,
    pub start_time: String,
    pub end_time: String,
    pub duration: i64,
    pub execution: Execution,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub resolvers: Vec<::serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    #[serde(rename = "operationName")]
    operation_name: String,
    query: String,
    variables: Variables,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    pub data: Data,
    pub extensions: Extensions,
}

pub async fn get_locations(
    lat: f32,
    long: f32,
    rad: f32,
) -> Result<Vec<SearchPharmaciesNearPoint>, Box<dyn std::error::Error>> {
    let post_data = PostData {
        operation_name: "SearchPharmaciesNearPointWithCovidVaccineAvailability".into(),
        variables: Variables {
            radius: rad,
            latitude: lat,
            longitude: long,
        },
        query: "query SearchPharmaciesNearPointWithCovidVaccineAvailability($latitude: Float!, $longitude: Float!, $radius: Int! = 100) {searchPharmaciesNearPoint(latitude: $latitude, longitude: $longitude, radius: $radius) {distance location { locationId name nickname phoneNumber businessCode isCovidVaccineAvailable covidVaccineEligibilityTerms address { line1 line2 city state zip latitude longitude __typename } __typename } __typename }}".into(),
    };

    let resp = reqwest::Client::new()
        .post("https://www.hy-vee.com/my-pharmacy/api/graphql")
        .json(&post_data)
        .send()
        .await?
        .json::<ResponseData>()
        .await?;

    Ok(resp.data.search_pharmacies_near_point)
}
