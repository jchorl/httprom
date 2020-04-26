#[macro_use]
extern crate prometheus;

use clap::Clap;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use http::Method;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clap)]
struct Opts {
    #[clap(short = "X", long = "request", default_value = "GET")]
    method: String,
    #[clap(short = "H", long = "header", multiple = true)]
    headers: Vec<String>,
    #[clap(long = "metrics-prefix", required = true)]
    metrics_prefix: String,
    #[clap(long = "prometheus-push-addr", required = true)]
    metrics_addr: String,
    input: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let mut headers = HeaderMap::new();
    for header in opts.headers {
        let mut s = header.split(": ");
        let k = s.next().unwrap();
        let v = s.next().unwrap();
        headers.insert(
            HeaderName::from_str(k).unwrap(),
            HeaderValue::from_str(v).unwrap(),
        );
    }

    let resp = reqwest::Client::new()
        .request(Method::from_str(&opts.method).unwrap(), &opts.input)
        .headers(headers)
        .send()
        .await?;

    let _ = meter_status(&opts.metrics_addr, &opts.metrics_prefix, resp.status());
    Ok(())
}

fn meter_status(addr: &str, prefix: &str, status: http::StatusCode) -> prometheus::Result<()> {
    let res_gauge = prometheus::register_gauge!(
        format!("{}_last_completed_epoch_seconds", prefix),
        String::from("The time of last completion")
    )
    .unwrap();
    res_gauge.set(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64,
    );
    prometheus::push_metrics(
        "httprom",
        labels! {"status".to_owned() => status.as_str().to_owned(),},
        addr,
        prometheus::gather(),
        None,
    )
}
