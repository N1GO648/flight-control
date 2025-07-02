use actix_web::{web, HttpResponse, Responder};
use rusqlite::{params, Connection};
use std::sync::Mutex;
use crate::models::{Flight, Pilot, Aircraft, WeatherRequest};
use chrono::Utc;
use std::env;
use serde_json::Value;

// -- Flight scheduling --

pub async fn schedule_flight(
    flight: web::Json<Flight>,
    data: web::Data<Mutex<Connection>>
) -> impl Responder {
    let flight = flight.into_inner();
    if flight.flight_plan.trim().is_empty() {
        return HttpResponse::BadRequest().body("Flight plan is required.");
    }
    if flight.is_in_past() {
        return HttpResponse::BadRequest().body("Departure time cannot be in the past.");
    }

    let conn = data.lock().unwrap();
    let res = conn.execute(
        "INSERT INTO flight (flight_id, pilot_id, aircraft_id, flight_plan, departure_time)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![flight.flight_id, flight.pilot_id, flight.aircraft_id,
                flight.flight_plan, flight.departure_time.to_rfc3339()],
    );

    match res {
        Ok(_) => HttpResponse::Created().json(flight),
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint") {
                HttpResponse::Conflict().body("Flight ID already exists.")
            } else {
                HttpResponse::InternalServerError().body("Failed to schedule flight.")
            }
        }
    }
}

// -- View upcoming flights --

pub async fn view_scheduled_flights(
    data: web::Data<Mutex<Connection>>
) -> impl Responder {
    let now = Utc::now().to_rfc3339();
    let conn = data.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT flight_id, pilot_id, aircraft_id, flight_plan, departure_time
         FROM flight WHERE departure_time >= ?"
    ).unwrap();

    let flights = stmt.query_map([now], |row| {
        Ok(Flight {
            flight_id: row.get(0)?,
            pilot_id: row.get(1)?,
            aircraft_id: row.get(2)?,
            flight_plan: row.get(3)?,
            departure_time: row.get::<_, String>(4)?.parse().unwrap(),
        })
    }).unwrap()
      .filter_map(Result::ok)
      .collect::<Vec<_>>();

    HttpResponse::Ok().json(flights)
}

// -- View past (history) flights --

pub async fn view_flight_history(
    data: web::Data<Mutex<Connection>>
) -> impl Responder {
    let now = Utc::now().to_rfc3339();
    let conn = data.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT flight_id, pilot_id, aircraft_id, flight_plan, departure_time
         FROM flight WHERE departure_time < ?"
    ).unwrap();

    let history = stmt.query_map([now], |row| {
        Ok(Flight {
            flight_id: row.get(0)?,
            pilot_id: row.get(1)?,
            aircraft_id: row.get(2)?,
            flight_plan: row.get(3)?,
            departure_time: row.get::<_, String>(4)?.parse().unwrap(),
        })
    }).unwrap()
      .filter_map(Result::ok)
      .collect::<Vec<_>>();

    HttpResponse::Ok().json(history)
}

// -- Update flight plan --

pub async fn update_flight_plan(
    path: web::Path<u32>,
    body: web::Json<String>,
    data: web::Data<Mutex<Connection>>
) -> impl Responder {
    let flight_id = path.into_inner();
    let new_plan = body.into_inner();

    if new_plan.trim().is_empty() {
        return HttpResponse::BadRequest().body("New flight plan cannot be empty.");
    }

    let conn = data.lock().unwrap();
    let affected = conn.execute(
        "UPDATE flight SET flight_plan = ?1 WHERE flight_id = ?2",
        params![new_plan, flight_id],
    ).unwrap();

    if affected == 1 {
        HttpResponse::Ok().body("Flight plan updated.")
    } else {
        HttpResponse::NotFound().body("Flight not found.")
    }
}

// -- Cancel (delete) a flight --

pub async fn cancel_flight(
    path: web::Path<u32>,
    data: web::Data<Mutex<Connection>>
) -> impl Responder {
    let flight_id = path.into_inner();
    let conn = data.lock().unwrap();
    let affected = conn.execute(
        "DELETE FROM flight WHERE flight_id = ?1",
        params![flight_id],
    ).unwrap();

    if affected == 1 {
        HttpResponse::Ok().body("Flight cancelled.")
    } else {
        HttpResponse::NotFound().body("Flight not found.")
    }
}

// -- Pilots endpoints --

pub async fn list_pilots(data: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = data.lock().unwrap();
    let mut stmt = conn.prepare("SELECT pilot_id, name, license_number FROM pilot").unwrap();
    let pilots = stmt.query_map([], |row| {
        Ok(Pilot {
            pilot_id: row.get(0)?,
            name: row.get(1)?,
            license_number: row.get(2)?,
        })
    }).unwrap().filter_map(Result::ok).collect::<Vec<_>>();
    HttpResponse::Ok().json(pilots)
}

// -- Aircraft endpoints --

pub async fn list_aircraft(data: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = data.lock().unwrap();
    let mut stmt = conn.prepare("SELECT aircraft_id, model, capacity FROM aircraft").unwrap();
    let craft = stmt.query_map([], |row| {
        Ok(Aircraft {
            aircraft_id: row.get(0)?,
            model: row.get(1)?,
            capacity: row.get(2)?,
        })
    }).unwrap().filter_map(Result::ok).collect::<Vec<_>>();
    HttpResponse::Ok().json(craft)
}

// -- Real-time weather --

pub async fn retrieve_weather(info: web::Query<WeatherRequest>) -> impl Responder {
    // Load your key from .env / env
    let api_key = match env::var("OPENWEATHER_API_KEY") {
        Ok(k) => k,
        Err(_) => return HttpResponse::InternalServerError().body("Missing API key"),
    };
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        info.latitude, info.longitude, api_key
    );

    // Async request
    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(json) => HttpResponse::Ok().json(json),
            Err(_) => HttpResponse::InternalServerError().body("Failed to parse weather JSON"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Weather service error"),
    }
}