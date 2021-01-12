extern crate prometheus;
extern crate tiny_http;

use prometheus::{CounterVec, Encoder, Opts, Registry, TextEncoder};
use tiny_http::{Response, Server};

fn main() {
    let counter_opts = Opts::new("hits", "Requests to path");
    let hits = CounterVec::new(counter_opts, &["path"]).unwrap();
    let encoder = TextEncoder::new();

    let r = Registry::new();
    r.register(Box::new(hits.clone())).unwrap();

    /* TODO: listen on a different port for prom */
    let server = Server::http("0.0.0.0:8080").expect("Can't start server");
    println!("Listening on {:?}", server.server_addr());

    for request in server.incoming_requests() {
        println!("{:?}", request);

        let path = request.url();
        if path == "/metrics" {
            let metric_families = r.gather();
            let mut buf = vec![];
            encoder.encode(&metric_families, &mut buf).unwrap();

            let response = Response::from_data(buf);
            let _ = request.respond(response);
        } else {
            println!("hit {}", path);

            hits.with_label_values(&[path]).inc();

            let response = Response::from_string(path);
            let _ = request.respond(response);
        }
    }
}
