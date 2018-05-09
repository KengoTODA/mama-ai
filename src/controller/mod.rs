mod aqi;
mod index;

use router::Router;

pub fn initialize() -> Router {
    let mut router: Router = Router::new();
    router.get("/", index::index, "index");
    router.post("/aqi/", aqi::parse, "parse");
    router.delete("/aqi/", aqi::prune, "prune");
    router
}
