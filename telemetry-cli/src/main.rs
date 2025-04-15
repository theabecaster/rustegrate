use chrono::Utc;
use clap::{Parser, Subcommand};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time;

#[derive(Parser)]
#[clap(
    name = "telemetry-cli",
    about = "CLI for sending simulated telemetry data"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a single telemetry data point
    Send {
        /// Server URL, e.g., http://localhost:8080
        #[clap(short, long, default_value = "http://localhost:8080")]
        url: String,

        /// Device ID to use
        #[clap(short, long, default_value = "device-001")]
        device_id: String,

        /// Temperature value (°C)
        #[clap(short, long)]
        temperature: Option<f32>,

        /// Humidity value (%)
        #[clap(short, long)]
        humidity: Option<f32>,

        /// Pressure value (hPa)
        #[clap(short, long)]
        pressure: Option<f32>,
    },

    /// Simulate a device sending telemetry data continuously
    Simulate {
        /// Server URL, e.g., http://localhost:8080
        #[clap(short, long, default_value = "http://localhost:8080")]
        url: String,

        /// Device ID to use
        #[clap(short, long, default_value = "device-001")]
        device_id: String,

        /// Interval between data points in seconds
        #[clap(short, long, default_value = "5")]
        interval: u64,

        /// Number of data points to send (default: unlimited)
        #[clap(short, long)]
        count: Option<usize>,

        /// Base temperature (random variations will be added)
        #[clap(long, default_value = "22.0")]
        base_temperature: f32,

        /// Base humidity (random variations will be added)
        #[clap(long, default_value = "45.0")]
        base_humidity: f32,

        /// Base pressure (random variations will be added)
        #[clap(long, default_value = "1013.0")]
        base_pressure: f32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct TelemetryPayload {
    device_id: String,
    temperature: f32,
    humidity: Option<f32>,
    pressure: Option<f32>,
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
            url,
            device_id,
            temperature,
            humidity,
            pressure,
        } => {
            // Generate random temperature if not provided
            let temp = temperature.unwrap_or_else(|| {
                let mut rng = rand::thread_rng();
                20.0 + rng.gen_range(-10.0..10.0)
            });

            let payload = TelemetryPayload {
                device_id,
                temperature: temp,
                humidity,
                pressure,
            };

            let response = send_telemetry(&client, &url, payload).await?;
            println!("Telemetry data sent successfully. ID: {}", response.id);
        }

        Commands::Simulate {
            url,
            device_id,
            interval,
            count,
            base_temperature,
            base_humidity,
            base_pressure,
        } => {
            println!("Simulating device telemetry for device: {}", device_id);
            println!("Sending data every {} seconds to {}", interval, url);
            println!("Press Ctrl+C to stop");

            let mut rng = rand::thread_rng();
            let mut sent_count = 0;

            loop {
                // Generate random variations
                let temp = base_temperature + rng.gen_range(-3.0..3.0);
                let humidity = base_humidity + rng.gen_range(-5.0..5.0);
                let pressure = base_pressure + rng.gen_range(-10.0..10.0);

                let payload = TelemetryPayload {
                    device_id: device_id.clone(),
                    temperature: temp,
                    humidity: Some(humidity),
                    pressure: Some(pressure),
                };

                match send_telemetry(&client, &url, payload).await {
                    Ok(response) => {
                        println!(
                            "[{}] Sent: temp={:.1}°C, humidity={:.1}%, pressure={:.1}hPa (ID: {})",
                            Utc::now().format("%Y-%m-%d %H:%M:%S"),
                            temp,
                            humidity,
                            pressure,
                            response.id
                        );
                    }
                    Err(e) => {
                        eprintln!("Error sending telemetry: {}", e);
                    }
                }

                sent_count += 1;

                // Check if we've reached the desired count
                if let Some(max_count) = count {
                    if sent_count >= max_count {
                        println!("Sent {} data points, exiting", max_count);
                        break;
                    }
                }

                // Wait for the next interval
                time::sleep(Duration::from_secs(interval)).await;
            }
        }
    }

    Ok(())
}

async fn send_telemetry(
    client: &Client,
    base_url: &str,
    payload: TelemetryPayload,
) -> Result<ApiResponse, Box<dyn Error>> {
    let url = format!("{}/api/v1/telemetry", base_url);

    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?
        .error_for_status()?;

    let api_response = response.json::<ApiResponse>().await?;
    Ok(api_response)
}
