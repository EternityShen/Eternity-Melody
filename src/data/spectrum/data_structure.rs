use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Chart {
    pub metadata: Metadata,
    pub notes: Vec<Note>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub title: String,
    pub bpm: f64,
    pub offset: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    pub time: f64,
    pub lane: usize,
}
