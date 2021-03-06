use actix_web::{App, get, HttpResponse, HttpServer, web::Data};
use prometheus::{Encoder, TextEncoder, Counter, Registry};

#[get("/metrics")]
pub async fn metrics(registry: Data<Registry>) -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buf = Vec::new();
    if let Err(err) = encoder.encode(&metric_families[..], &mut buf) {
        return HttpResponse::InternalServerError().body(err.to_string())
    }

    let body = String::from_utf8(buf).unwrap_or_default();
    HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, prometheus::TEXT_FORMAT)
        .body(body)
}

#[get("/greet")]
pub async fn greet(counter: Data<Counter>) -> HttpResponse {
    counter.inc();
    HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, "text/plain")
        .body("hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Counter::new("world", "world help").expect("counter");
    let registry = Registry::new();
    registry.register(Box::new(counter.clone())).expect("register counter");

    HttpServer::new(move || App::new()
        .data(registry.clone())
        .data(counter.clone())
        .service(metrics)
        .service(greet))
        .bind("localhost:8080")?
        .run()
        .await
}
