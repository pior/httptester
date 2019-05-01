#[macro_use]
extern crate log;
extern crate env_logger;

extern crate actix_web;
use actix_web::{server, App, HttpRequest, HttpResponse, Responder, http::Method};

use std::path::PathBuf;
// use std::cell::Cell;
use std::sync::{Arc, Mutex};


struct RequestState {
    counter: Arc<Mutex<usize>>,
}

fn ok_handler(_req: &HttpRequest<RequestState>) -> impl Responder {
    HttpResponse::Ok()
}

fn show_calls(req: &HttpRequest<RequestState>) -> impl Responder {
    let pcount = req.state().counter.lock().unwrap();

    format!("count={}", *pcount)
}

fn record_call(req: &HttpRequest<RequestState>) -> impl Responder {
    let path: PathBuf = req.match_info().query("tail").unwrap();

    let mut pcount = req.state().counter.lock().unwrap();
    *pcount += 1;

    HttpResponse::Ok()
}

fn main() {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    info!("Starting");

    let c = Arc::new(Mutex::new(0));

    server::new(move || {

        App::with_state(RequestState { counter: Arc::clone(&c) })

            .resource("/", |r| r.method(Method::GET).f(ok_handler))

            .resource("/_stats", |r| r.method(Method::GET).f(show_calls))

            .resource("/test/{tail:.*}", |r| r.method(Method::GET).f(record_call))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
