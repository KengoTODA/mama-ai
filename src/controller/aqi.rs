use hyper::Client;
use hyper::net::HttpsConnector;
use hyper::header::Connection;
use hyper_native_tls::NativeTlsClient;
use iron::{IronResult, Request, Response};
use iron::status;
use std::env;
use std::io::Read;

use registry;

pub fn parse(req: &mut Request) -> IronResult<Response> {
    let mut buffer = String::new();
    req.body.read_to_string(&mut buffer).unwrap();
    let quality: i16 = buffer.parse().unwrap();
    if quality >= 100 {
        try_submit(quality);
    }
    let r = registry::connect();
    r.insert(quality).unwrap();
    let resp = Response::with((status::Ok, format!("Air quality is {}!", quality)));
    Ok(resp)
}

pub fn prune(_req: &mut Request) -> IronResult<Response> {
    let r = registry::connect();
    let deleted = r.prune();
    let resp = Response::with((
        status::Ok,
        format!("Deleted {:?} records successfully", deleted.unwrap()),
    ));
    Ok(resp)
}

fn try_submit(quality: i16) {
    match env::var("IFTTT_KEY") {
        Ok(key) => {
            let url = format!(
                "https://maker.ifttt.com/trigger/mama_ai_air_quality/with/key/{}?value1={}",
                key, quality
            );
            submit(&url);
        }
        Err(e) => warn!("Failed to get IFTTT key: {}", e),
    }
}

// https://stackoverflow.com/a/14189088/814928
fn submit(url: &str) {
    info!("submit to {}", url);
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    client.get(url).header(Connection::close()).send().unwrap();
}
