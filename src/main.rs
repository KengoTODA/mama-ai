extern crate chrono;
extern crate iron;
extern crate router;

extern crate env_logger;
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate log;

use iron::Iron;

mod controller;
mod registry;
mod value;

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    env_logger::init().unwrap();
    info!("starting up");
    Iron::new(controller::initialize())
        .http(("0.0.0.0", get_server_port()))
        .unwrap();
}
