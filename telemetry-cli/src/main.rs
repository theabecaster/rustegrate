use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use rand::prelude::*;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, time::Duration};
use tokio::time::sleep;

/// CLI for simulating iRacing telemetry data
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a single telemetry reading
    Send {
        /// Driver ID for this telemetry data
        #[arg(short, long)]
        driver: String,

        /// Session ID for this telemetry data
        #[arg(short, long)]
        session: String,

        /// Session type (Practice, Qualifying, Race)
        #[arg(short, long)]
        session_type: String,

        /// Track name
        #[arg(short, long)]
        track: String,

        /// Car model
        #[arg(short, long)]
        car: String,

        /// Vehicle speed in km/h
        #[arg(short, long)]
        speed: f32,

        /// Engine RPM
        #[arg(short = 'r', long)]
        rpm: Option<f32>,

        /// Current gear (-1 to 8)
        #[arg(short, long)]
        gear: Option<i8>,

        /// Current lap number
        #[arg(short, long)]
        lap: Option<u32>,

        /// Fuel level (percentage)
        #[arg(short = 'f', long)]
        fuel: Option<f32>,
    },

    /// Continuously simulate telemetry data
    Simulate {
        /// Driver ID for this telemetry data
        #[arg(short, long)]
        driver: String,

        /// Session ID (will be randomly generated if not provided)
        #[arg(short, long)]
        session: Option<String>,

        /// Session type (Practice, Qualifying, Race)
        #[arg(short, long, default_value = "Race")]
        session_type: String,

        /// Track name
        #[arg(short, long, default_value = "Daytona")]
        track: String,

        /// Car model
        #[arg(short, long, default_value = "NASCAR Cup Series Next Gen Chevrolet Camaro ZL1")]
        car: String,

        /// Interval between readings in seconds
        #[arg(short, long, default_value_t = 1)]
        interval: u64,

        /// Number of laps to simulate
        #[arg(short, long, default_value_t = 10)]
        laps: u32,
    },
}

/// Request model for creating telemetry
#[derive(Serialize, Deserialize, Debug)]
struct TelemetryRequest {
    driver_id: String,
    session_id: String,
    session_type: String,
    track_name: String,
    car_name: String,
    speed: f32,
    rpm: f32,
    gear: i8,
    current_lap: u32,
    last_lap_time: Option<f32>,
    best_lap_time: Option<f32>,
    fuel_level: f32,
    throttle_position: f32,
    brake_position: f32,
    clutch_position: f32,
    steering_angle: f32,
    tire_temps_fl: f32,
    tire_temps_fr: f32,
    tire_temps_rl: f32,
    tire_temps_rr: f32,
    lateral_g: f32,
    longitudinal_g: f32,
    vertical_g: f32,
    track_position: f32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    timestamp: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::Send {
            driver,
            session,
            session_type,
            track,
            car,
            speed,
            rpm,
            gear,
            lap,
            fuel,
        } => {
            let payload = TelemetryRequest {
                driver_id: driver,
                session_id: session,
                session_type,
                track_name: track,
                car_name: car,
                speed,
                rpm: rpm.unwrap_or(0.0),
                gear: gear.unwrap_or(0),
                current_lap: lap.unwrap_or(0),
                last_lap_time: None,
                best_lap_time: None,
                fuel_level: fuel.unwrap_or(0.0),
                throttle_position: 0.0,
                brake_position: 0.0,
                clutch_position: 0.0,
                steering_angle: 0.0,
                tire_temps_fl: 0.0,
                tire_temps_fr: 0.0,
                tire_temps_rl: 0.0,
                tire_temps_rr: 0.0,
                lateral_g: 0.0,
                longitudinal_g: 0.0,
                vertical_g: 0.0,
                track_position: 0.0,
                position_x: 0.0,
                position_y: 0.0,
                position_z: 0.0,
                timestamp: None,
            };

            let response = send_telemetry(&client, payload).await?;
            println!("Telemetry data sent successfully. ID: {}", response.id);
        }

        Commands::Simulate {
            driver,
            session,
            session_type,
            track,
            car,
            interval,
            laps,
        } => {
            println!("Simulating telemetry for driver: {}", driver);
            println!("Sending data every {} seconds", interval);
            println!("Press Ctrl+C to stop");

            let mut rng = rand::thread_rng();
            let mut sent_count = 0;

            loop {
                let payload = TelemetryRequest {
                    driver_id: driver.clone(),
                    session_id: session.clone().unwrap_or_else(|| {
                        format!("session-{}", rng.gen_range(1..=100))
                    }),
                    session_type,
                    track_name: track.clone(),
                    car_name: car.clone(),
                    speed: rng.gen_range(50.0..200.0),
                    rpm: Some(rng.gen_range(1000.0..8000.0)),
                    gear: Some(rng.gen_range(-1..=8) as i8),
                    current_lap: rng.gen_range(1..=20) as u32,
                    last_lap_time: None,
                    best_lap_time: None,
                    fuel_level: rng.gen_range(10.0..=100.0),
                    throttle_position: rng.gen_range(0.0..=1.0),
                    brake_position: rng.gen_range(0.0..=1.0),
                    clutch_position: rng.gen_range(0.0..=1.0),
                    steering_angle: rng.gen_range(-30.0..30.0),
                    tire_temps_fl: rng.gen_range(50.0..100.0),
                    tire_temps_fr: rng.gen_range(50.0..100.0),
                    tire_temps_rl: rng.gen_range(50.0..100.0),
                    tire_temps_rr: rng.gen_range(50.0..100.0),
                    lateral_g: rng.gen_range(-2.0..2.0),
                    longitudinal_g: rng.gen_range(-2.0..2.0),
                    vertical_g: rng.gen_range(-2.0..2.0),
                    track_position: rng.gen_range(0.0..100.0),
                    position_x: rng.gen_range(-100.0..100.0),
                    position_y: rng.gen_range(-100.0..100.0),
                    position_z: rng.gen_range(-100.0..100.0),
                    timestamp: None,
                };

                match send_telemetry(&client, payload).await {
                    Ok(response) => {
                        println!(
                            "[{}] Sent: driver={}, session={}, session_type={}, track={}, car={}, speed={:.1} km/h, rpm={:.0} RPM, gear={}, lap={}, fuel={:.1}% (ID: {})",
                            Utc::now().format("%Y-%m-%d %H:%M:%S"),
                            payload.driver_id,
                            payload.session_id,
                            payload.session_type,
                            payload.track_name,
                            payload.car_name,
                            payload.speed,
                            payload.rpm,
                            payload.gear,
                            payload.current_lap,
                            payload.fuel_level,
                            response.id
                        );
                    }
                    Err(e) => {
                        eprintln!("Error sending telemetry: {}", e);
                    }
                }

                sent_count += 1;

                if sent_count >= laps {
                    println!("Sent {} data points, exiting", laps);
                    break;
                }

                sleep(Duration::from_secs(interval)).await;
            }
        }
    }

    Ok(())
}

async fn send_telemetry(
    client: &Client,
    payload: TelemetryRequest,
) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!("{}/api/v1/telemetry", env::var("SERVER_URL").expect("SERVER_URL must be set"));

    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    let api_response = response.json::<ApiResponse>().await?;
    Ok(api_response)
}
