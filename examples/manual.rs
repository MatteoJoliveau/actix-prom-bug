use actix_web::{App, get, HttpResponse, HttpServer, web::Data};
use opentelemetry::{global, KeyValue, metrics::MetricsError};
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{Encoder, TextEncoder};

#[get("/metrics")]
pub async fn metrics(exporter: Data<PrometheusExporter>) -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = exporter.registry().gather();
    let mut buf = Vec::new();
    if let Err(err) = encoder.encode(&metric_families[..], &mut buf) {
        global::handle_error(MetricsError::Other(err.to_string()));
    }

    let body = String::from_utf8(buf).unwrap_or_default();
    HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, prometheus::TEXT_FORMAT)
        .body(body)
}

#[get("/greet")]
pub async fn greet() -> HttpResponse {
    let meter = global::meter("greeter");
    let counter = meter.u64_counter("greets_total").init();
    counter.add(1, &[KeyValue::new("name", "world")]);
    HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, "text/plain")
        .body("hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let exporter = opentelemetry_prometheus::exporter().init();
    HttpServer::new(move || App::new()
        .data(exporter.clone())
        .service(metrics)
        .service(greet))
        .bind("localhost:8080")?
        .run()
        .await
}
