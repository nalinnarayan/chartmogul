#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;


use rocket::request::Form;
use std::env;
use reqwest;
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION, ACCEPT};
use rocket_contrib::json::Json;
use dotenv::dotenv;

//Structure for handling MRR data from ChartMogul
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRR{
	date: String,
    mrr: f64,
}

//Structure for receiving MRR retrieval request from UI Form
#[derive(FromForm, Debug, Clone, Serialize, Deserialize)]
pub struct ToRetrieveMRR{
  start_date: String,
  end_date: String,
  interval: String,
}

//Structure for handling MRR data received from ChartMogul
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromRetrieveMRR{
  entries: Vec<MRR>,
}


//this route receives the MRR form data from the UI, calls the 
//ChartMogul metrics API's retrieve-MRR endpoint, and returns MRR 
//with date.
#[post("/getmrr", data = "<mrrparams>")]
fn get_mrr(mrrparams: Form<ToRetrieveMRR>) -> Result<Json<Vec<MRR>>,Error>{
	dotenv().ok();
  //Getting the Auth headers ready
  let req_client = reqwest::blocking::Client::new();
  let chartmogul_base_api_url = env::var("CHARTMOGUL_BASE_API_URL").unwrap();
  let chartmogul_api_key = env::var("CHARTMOGUL_API_KEY").unwrap();
  let chartmogul_authorization_header = format!("{}{}", "Basic ", chartmogul_api_key);
  let chartmogul_authorization_header_str = chartmogul_authorization_header.as_str();
  //Inserting all headers
  let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
	headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
	headers.insert(AUTHORIZATION, HeaderValue::from_str(chartmogul_authorization_header_str).unwrap());
  
  //Getting the request url and query parameters ready
  let to_chartmogul_retrieve_mrr_url = format!("{}{}", chartmogul_base_api_url.as_str(), "/metrics/mrr?");
  let parameters = &[
			("start-date", mrrparams.start_date.as_str()),
			("end-date", mrrparams.end_date.as_str()),
      		("interval", mrrparams.interval.as_str()),
		];
  let url = to_chartmogul_retrieve_mrr_url + &serde_urlencoded::to_string(parameters).unwrap();
  //Making the call to ChartMogul Metrics API's Retrieve MRR endpoint
  let res = req_client.get(url.as_str())
      .headers(headers)
      .send()?;
  
  let deserialized_res: FromRetrieveMRR = res.json().unwrap();
  Ok(Json(deserialized_res.entries))

}
fn main() {
    rocket::ignite()
    .mount("/", routes![get_mrr])
    .launch();
}
