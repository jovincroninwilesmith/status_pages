extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Current {
  percentage: f64,
  level: String,
  title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bank {
  current: Current,
  is_internal: bool,
  historical: Vec<Current>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Banks {
  nfcu: Bank,
  citi: Bank,
  simple: Bank,
  wells: Bank,
  usaa: Bank,
  td: Bank,
  #[serde(rename = "fifth-third")]
  _fifthThird: Bank,
  discover: Bank,
  schwab: Bank,
  site: Bank,
  amex: Bank,
  bofa: Bank,
  api: Bank,
  us: Bank,
  capone360: Bank,
  fidelity: Bank,
  tartan: Bank,
  suntrust: Bank,
  pnc: Bank,
  chase: Bank,
}

fn main() -> Result<(), reqwest::Error> {
    let mut res = reqwest::get("https://status.plaid.com/institutions/uptime")?;
    let body: Banks = res.json()?;
    
    println!("Body:\n{:?}", body);
    
    Ok(())
}


