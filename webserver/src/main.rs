extern crate actix;
extern crate actix_web;

use actix_web::{server, App, HttpRequest, Responder, HttpResponse};

fn main() {
    let sys = actix::System::new("sammy data");

    server::HttpServer::new(|| App::new().resource("/", |r| r.f(|_| HttpResponse::Ok())))
        .bind("127.0.0.1:18080")
        .unwrap()
        .start();

    let _ = sys.run();
}
