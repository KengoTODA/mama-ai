extern crate iron;
extern crate router;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate hyper_native_tls;

use std::env;
use std::io::Read;
use iron::{Iron, Request, Response, IronResult};
use router::Router;
use iron::status;
use hyper::{Client};
use hyper::net::HttpsConnector;
use hyper::header::Connection;
use hyper_native_tls::NativeTlsClient;

// Serves a string to the user.  Try accessing "/".
fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, "Hello world!"));
    Ok(resp)
}

// Serves a customized string to the user.  Try accessing "/world".
fn parse(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::new();
    req.body.read_to_string(&mut buffer).unwrap();
    let quality = parse_air_quality(&buffer).unwrap();
    if quality >= 100 {
      try_submit(quality);
    }
    let resp = Response::with((status::Ok, format!("Air quality is {}!", quality)));
    Ok(resp)
}

//  https://twitter.com/CGShanghaiAir
fn parse_air_quality(text: &str) -> Option<i32> {
  let taken = text.split("; ").nth(3);
  match taken {
    Some(s) => match s.parse::<i32>() {
      Ok(n) => Some(n),
      _ => None
    },
    None => None
  }
}

#[test]
fn test_parse_air_quality() {
  assert_eq!(
    parse_air_quality("11-05-2017 07:00; PM2.5; 26.0; 80; Moderate (at 24-hour exposure at this level)"),
    Some(80));
}

#[test]
fn test_parse_no_data() {
  assert_eq!(
    parse_air_quality("11-04-2017 03:00; PM2.5; No Data"),
    None);
}

fn try_submit(quality: i32) {
  match env::var("IFTTT_KEY") {
    Ok(key) => {
      let url = format!("https://maker.ifttt.com/trigger/mama_ai_air_quality/with/key/{}?value1={}", key, quality);
      submit(&url);
    },
    Err(e) => warn!("Failed to get IFTTT key: {}", e)
  }
}

// https://stackoverflow.com/a/14189088/814928
fn submit(url: &str) {
  info!("submit to {}", url);
  let tls = NativeTlsClient::new().unwrap();
  let connector = HttpsConnector::new(tls);
  let client = Client::with_connector(connector);

  client.get(url)
      .header(Connection::close())
      .send()
      .unwrap();
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    env_logger::init().unwrap();
    info!("starting up");

    // Set up our URL router.
    let mut router: Router = Router::new();
    router.get("/", hello, "index");
    router.post("/", parse, "parse");

    // Run the server.
    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}
