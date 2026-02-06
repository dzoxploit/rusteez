mod cli;
mod model;
mod checker;
mod metrics;


use anyhow::Result;
use cli::Args;
use checker::check_service;
use model::Service;
use reqwest::Client;
use std::fs;
use clap::Parser;
use crate::model::Report;


#[tokio::main]

async fn main() -> Result<()> {
    let args = Args::parse();

    metrics::serve(args.metrics_port);

    let content = fs::read_to_string(args.file)?;
    let services: Vec<Service> = serde_json::from_str(&content)?;

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;


    let futures = services.iter().map(|svc| {
       let cl = &client;
       async move {check_service(cl, svc, args.retries).await }
    });

    let results = futures::future::join_all(futures).await;

    let reports: Vec<Report> = results;

    println!("{}", serde_json::to_string_pretty(&reports).unwrap());

    Ok(())
}