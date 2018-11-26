use actix_web::{server, App, HttpResponse, HttpRequest, Error};
use actix_web::http::Method;
use futures::future::{result, FutureResult};

fn serve_index(req: &HttpRequest) -> FutureResult<HttpResponse, Error> {
    info!("Received request: {:?}", &req);
    result(Ok(HttpResponse::Ok()
        .content_type("text/json; charset=utf-8")
        .body("{ \"status\": \"Not yet!\"}")
    ))
}

pub fn start_web(host: &String) -> Result<(), Box<std::error::Error>> {
    info!("Synchronization rate requirements are go.");
    server::new(
        || App::new()
            .resource("/index.json", |r| r.method(Method::GET).a(serve_index)))
        .bind(host).unwrap()
        .shutdown_timeout(0)
        .run();
    Ok(())
}