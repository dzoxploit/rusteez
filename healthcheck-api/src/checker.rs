use std::time::{Duration, Instant};

use backoff::{future::retry, ExponentialBackoff};
use reqwest::Client;

use crate::model::{Report, Service};

pub async fn check_service(
    client: &Client,
    svc: &Service,
    retries: u32,
) -> Report {
    let operation = || async {
        let start = Instant::now();

        match client
            .get(&svc.url)
            .header("User-Agent", "healthcheck-api")
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let latency = start.elapsed().as_millis();
                Ok((status, latency))
            }
            Err(e) => Err(backoff::Error::transient(e)),
        }
    };

    let policy = ExponentialBackoff {
        initial_interval: Duration::from_millis(300),
        max_interval: Duration::from_secs(2),
        max_elapsed_time: Some(Duration::from_secs(2 * retries as u64)),
        ..Default::default()
    };

    match retry(policy, operation).await {
        Ok((status, latency)) => Report {
            name: svc.name.clone(),
            ok: status < 500,
            status: Some(status),
            latency_ms: latency,
        },
        Err(_) => Report {
            name: svc.name.clone(),
            ok: false,
            status: None,
            latency_ms: 0,
        },
    }
}
