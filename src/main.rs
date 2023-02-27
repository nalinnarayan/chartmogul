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

//Structure for sending MRR retrieval request to ChartMogul
#[derive(FromForm, Debug, Clone, Serialize, Deserialize)]
pub struct ToRetrieveMRR{
  start_date: String,
  end_date: String,
  interval: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromRetrieveMRR{
  entries: Vec<MRR>,
}



//#[post("/getmrr", data = "<mrrparams>")]
#[get("/getmrr")]
//fn get_mrr(mrrparams: Form<ToRetrieveMRR>) -> Result<Json<Vec<MRR>>,Error>{
fn get_mrr() -> Result<Json<Vec<MRR>>,Error>{
	dotenv().ok();
  let req_client = reqwest::blocking::Client::new();
  let chartmogul_base_api_url = env::var("CHARTMOGUL_BASE_API_URL").unwrap();
  let chartmogul_api_key = env::var("CHARTMOGUL_API_KEY").unwrap();
  let chartmogul_authorization_header = format!("{}{}", "Basic ", chartmogul_api_key);
  let chartmogul_authorization_header_str = chartmogul_authorization_header.as_str();
  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
	headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
	headers.insert(AUTHORIZATION, HeaderValue::from_str(chartmogul_authorization_header_str).unwrap());
  /*let mrr_params = ToRetrieveMRR{
    start_date: mrrparams.start_date.clone(),
    end_date: mrrparams.end_date.clone(),
    interval: mrrparams.interval.clone(),
  };*/
  let to_chartmogul_retrieve_mrr_url = format!("{}{}", chartmogul_base_api_url.as_str(), "/metrics/mrr?");
  let parameters = &[
			("start-date", "2019-01-01"),
			("end-date", "2019-04-30"),
      ("interval", "month"),
		];
  /*let parameters = &[
			("start-date", mrrparams.start_date.as_str()),
			("end-date", mrrparams.end_date.as_str()),
      ("interval", mrrparams.interval.as_str()),
		];*/
	let url = to_chartmogul_retrieve_mrr_url + &serde_urlencoded::to_string(parameters).unwrap();
  let res = req_client.get(url.as_str())
      .headers(headers)
      .send()?;
  /*let res = req_client.post(to_chartmogul_retrieve_mrr_url.as_str())
			.headers(headers)
			.json(&mrr_params)
			.send()?;*/
  let deserialized_res: FromRetrieveMRR = res.json().unwrap();
  Ok(Json(deserialized_res.entries))

}
fn main() {
    rocket::ignite()
    .mount("/", routes![get_mrr])
    .launch();
}
