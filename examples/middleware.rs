use actix_web::{dev, App, HttpServer, get, HttpResponse};
use actix_web_opentelemetry::RequestMetrics;
use opentelemetry::{global, KeyValue};

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
    let meter = global::meter("actix_web");

    // Optional predicate to determine which requests render the prometheus metrics
    let metrics_route = |req: &dev::ServiceRequest| {
        req.path() == "/metrics" && req.method() == http::Method::GET
    };

    // Request metrics middleware
    let request_metrics = RequestMetrics::new(meter, Some(metrics_route), Some(exporter));

    // Run actix server, metrics are now available at http://localhost:8080/metrics
    HttpServer::new(move || App::new().wrap(request_metrics.clone()).service(greet))
        .bind("localhost:8080")?
        .run()
        .await
}
