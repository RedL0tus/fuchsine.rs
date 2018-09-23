use actix_web::{server, App, HttpResponse, HttpRequest, Error};
use actix_web::http::Method;
use futures::future::{result, FutureResult};

fn index(req: &HttpRequest) -> FutureResult<HttpResponse, Error> {
    info!("Received request: {:?}", &req);
    result(Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<!DOCTYPE html><html><body style=\"background-color: Fuchsia;\"><p>Path: {}</p></body></html>", req.match_info().get("path").unwrap()))
    ))
}

pub fn start_web(host: &String) -> Result<(), Box<std::error::Error>> {
    info!("Synchronization rate requirements are go.");
    server::new(
        || App::new()
            .resource("/{path}", |r| r.method(Method::GET).a(index)))
        .bind(host).unwrap()
        .shutdown_timeout(0)
        .run();
    Ok(())
}