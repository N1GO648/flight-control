use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Flight {
    pub flight_id: u32,
    pub pilot_id: u32,
    pub aircraft_id: u32,
    pub flight_plan: String,
    pub departure_time: DateTime<Utc>,
}

impl Flight {
    pub fn is_in_past(&self) -> bool {
        self.departure_time < Utc::now()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pilot {
    pub pilot_id: u32,
    pub name: String,
    pub license_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct Aircraft {
    pub aircraft_id: u32,
    pub model: String,
    pub capacity: u32,
}

#[derive(Deserialize)]
pub struct WeatherRequest {
    pub latitude: f64,
    pub longitude: f64,
}