#[macro_use]
extern crate prometheus;

use clap::Clap;
use http::Method;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clap)]
struct Opts {
    #[clap(short = "X", long = "request", default_value = "GET")]
    method: String,
    input: String,
    #[clap(long = "metrics-prefix", required = true)]
    metrics_prefix: String,
    #[clap(long = "prometheus-push-addr", required = true)]
    metrics_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let resp = reqwest::Client::new()
        .request(Method::from_str(&opts.method).unwrap(), &opts.input)
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
        labels! {"status".to_owned() => status.to_string(),},
        addr,
        prometheus::gather(),
        None,
    )
}
