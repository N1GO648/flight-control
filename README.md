Flight Control Simulator

A simple Rust + Actix Web application for scheduling flights, managing flight plans, viewing upcoming and historical flights, and fetching real-time weather data.

Features
	•	Pilot & Aircraft management: Seeded tables for pilots and aircraft.
	•	Flight CRUD: Create, read (upcoming & history), update (plan), delete (cancel).
	•	Real-time weather: Async calls to OpenWeatherMap API.
	•	Front-end: Static index.html for interacting with the API.

Prerequisites
	•	Rust (1.68+)
	•	SQLite (bundled via rusqlite)
	•	OpenWeatherMap API Key

Getting Started
	1.	Clone the repo

git clone <your-repo-url>
cd practice-02


	2.	Create .env in project root:

OPENWEATHER_API_KEY=your_real_openweather_api_key_here_pleasemakeanaccountonthewebsite


	3.	Build and run

cargo run

The server listens on 127.0.0.1:8080.

Manual Testing (Smoke Tests)

1. List Pilots & Aircraft

curl http://127.0.0.1:8080/pilots
curl http://127.0.0.1:8080/aircraft

2. Schedule a Flight

curl -X POST http://127.0.0.1:8080/flights/schedule \
  -H "Content-Type: application/json" \
  -d '{
    "flight_id": 10,
    "pilot_id": 101,
    "aircraft_id": 202,
    "flight_plan": "Test Plan",
    "departure_time": "2030-01-01T10:00:00Z"
}'

Expect 201 Created and JSON of the new flight.

3. View Upcoming Flights

curl http://127.0.0.1:8080/flights/view

4. View Flight History

curl http://127.0.0.1:8080/flights/history

5. Update a Flight Plan

curl -X PUT http://127.0.0.1:8080/flights/10/plan \
  -H "Content-Type: application/json" \
  -d '"New Plan XYZ"'

6. Cancel a Flight

curl -X DELETE http://127.0.0.1:8080/flights/10

7. Fetch Weather

curl "http://127.0.0.1:8080/weather?latitude=1.3521&longitude=103.8198"

Front-end Testing
	1.	Open static/index.html in your browser.
	2.	Verify pilots and aircraft dropdowns load data.
	3.	Schedule, refresh, update, and cancel flights via the UI.
	4.	Fetch weather by entering coordinates.

Automated Tests
	1.	Write tests in tests/api.rs using actix_web::test.
	2.	Run all tests:

cargo test



⸻

Happy scheduling! Let me know if you run into any issues.
