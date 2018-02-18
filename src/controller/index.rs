use iron::{IronResult, Request, Response};
use registry;
use iron::status;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let r = registry::connect();
    let resp = Response::with((
        status::Ok,
        format!("Latest data: {:?}", r.select().unwrap().unwrap()),
    ));
    Ok(resp)
}
