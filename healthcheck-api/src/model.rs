use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Service {
    pub name: String,
    pub url: String,
}

#[derive(Serialize)]

pub struct Report {
    pub name: String,
    pub ok: bool,
    pub status : Option<u16>,
    pub latency_ms: u128,
}



