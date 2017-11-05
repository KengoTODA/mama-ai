extern crate iron;
extern crate router;

use std::env;
use std::io::Read;
use iron::{Iron, Request, Response, IronResult};
use router::Router;
use iron::status;

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

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    // Set up our URL router.
    let mut router: Router = Router::new();
    router.get("/", hello, "index");
    router.post("/", parse, "parse");

    // Run the server.
    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}
