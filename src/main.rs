use actix_web::{web, App, HttpServer};
use actix_files as fs;
use rusqlite::Connection;
use std::sync::Mutex;
use dotenv::dotenv;

mod handlers;
mod models;
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let conn = Connection::open_in_memory().unwrap();

    // Flights
    conn.execute_batch(r#"
        CREATE TABLE pilot (
            pilot_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            license_number TEXT NOT NULL
        );
        CREATE TABLE aircraft (
            aircraft_id INTEGER PRIMARY KEY,
            model TEXT NOT NULL,
            capacity INTEGER NOT NULL
        );
        CREATE TABLE flight (
            flight_id INTEGER PRIMARY KEY,
            pilot_id INTEGER NOT NULL,
            aircraft_id INTEGER NOT NULL,
            flight_plan TEXT NOT NULL,
            departure_time TEXT NOT NULL,
            FOREIGN KEY(pilot_id) REFERENCES pilot(pilot_id),
            FOREIGN KEY(aircraft_id) REFERENCES aircraft(aircraft_id)
        );
        -- seed sample pilots & aircraft
        INSERT INTO pilot VALUES (101, 'Alice Wong', 'LIC-ALW-001');
        INSERT INTO pilot VALUES (102, 'Bob Tan',   'LIC-BTN-002');

        INSERT INTO aircraft VALUES (202, 'Cessna 172', 4);
        INSERT INTO aircraft VALUES (203, 'Piper PA-28',4);
    "#).unwrap();

    let data = web::Data::new(Mutex::new(conn));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::scope("/flights")
                .route("/schedule", web::post().to(schedule_flight))
                .route("/view", web::get().to(view_scheduled_flights))
                .route("/history", web::get().to(view_flight_history))
                .route("/{id}/plan", web::put().to(update_flight_plan))
                .route("/{id}", web::delete().to(cancel_flight))
            )
            .service(web::scope("/pilots")
                .route("", web::get().to(list_pilots))
            )
            .service(web::scope("/aircraft")
                .route("", web::get().to(list_aircraft))
            )
            .route("/weather", web::get().to(retrieve_weather))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}