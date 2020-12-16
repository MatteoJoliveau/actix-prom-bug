# Weird bug in opentelemetry-prometheus

In the [`examples`](./examples) folder you can find two Actix Web executables, 
one using the [actix-web-opentelemetry](https://docs.rs/actix-web-opentelemetry/0.8.0/actix_web_opentelemetry/) middleware
and the other a manual implementation of the `/metrics` endpoint using [opentelemetry-prometheus](https://docs.rs/opentelemetry-prometheus/0.3.0/opentelemetry_prometheus/).

In both cases counters keep going up even if no HTTP calls are made.
N.B. the metrics need to be kicked-off once for this to start, you can do so by calling `curl http://localhost:8080/greet`.

See results:

[![Middleware](https://asciinema.org/a/EVw1aDP6vqLpqdiF126S6LVzs.svg)](https://asciinema.org/a/EVw1aDP6vqLpqdiF126S6LVzs)
[![Manual](https://asciinema.org/a/npt0o7HNCmgSUZ6A2k6hmLS6S.svg)](https://asciinema.org/a/npt0o7HNCmgSUZ6A2k6hmLS6S)
