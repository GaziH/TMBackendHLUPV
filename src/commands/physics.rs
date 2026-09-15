use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};
use tokio::time::interval;
use crate::messages::messages::{boost_entered, boost_success, precharge_done, stopped_critical, stopped_success};
use crate::structs::structs::{Data, Message, PayloadVehicle, State, Topic};

pub async fn tick_physics(vehicle: Arc<Mutex<PayloadVehicle>>, tx: broadcast::Sender<String>) {
    let mut ticker = interval(Duration::from_millis(250)); // Ticker to pause for 250ms
    let physical_time = 0.05; // Simulation speed is 5x slower, 0.25s/5 = 0.05s = 50ms

    loop{
        ticker.tick().await; // Wait for tick
        let mut vehicle = vehicle.lock().await; // Reserve vehicle
        vehicle.timestamp = chrono::Utc::now().to_rfc3339(); // Set timestamp of message

        match vehicle.state {
            State::IDLE => {
                // Do nothing
            }
            State::PRECHARGE => { // Increase Voltage steadily to max of 400V
                vehicle.voltage_v+=25.0;
                if(vehicle.voltage_v >=400.0){
                    vehicle.voltage_v=400.0;
                    vehicle.state = State::READY;
                    let msg: Message = precharge_done();
                    let _ = tx.send(serde_json::to_string(&msg).unwrap());
                }
            }
            State::READY => {
                // Do nothing
            }
            State::RUNNING =>{ // Calculate Position with constant Velocity

                if(vehicle.position_m>=2.0 && vehicle.position_m<4.0){ // If in Booster zone
                    vehicle.state = State::BOOSTING;
                    let msg = boost_entered();
                    let _ = tx.send(serde_json::to_string(&msg).unwrap());
                    continue;
                }
                if(vehicle.position_m>=50.0){ // If hits barrier
                    vehicle.state = State::BRAKING;
                    continue;
                }
                vehicle.position_m += (vehicle.velocity_kmh/3.6) * physical_time; // Position
            }
            State::BOOSTING => { // Calculate Acceleration, then update Velocity and Position

                if(vehicle.position_m>=4.0){
                    vehicle.current_a=0.0;
                    vehicle.acceleration_ms2=0.0;
                    vehicle.velocity_kmh=25.0;
                    vehicle.state = State::RUNNING;
                    let msg = boost_success(&vehicle);
                    let _ = tx.send(serde_json::to_string(&msg).unwrap());
                    continue;
                }

                let target_velocity_ms: f32 = 25.0/3.6; // 25km/h to m/s

                vehicle.acceleration_ms2=( target_velocity_ms.powf(2.0) - (vehicle.velocity_kmh/3.6).powf(2.0) ) // Acceleration
                    / (2.0*(4.0-vehicle.position_m) );
                
                vehicle.current_a = 200.0 * (std::f32::consts::PI * (vehicle.position_m - 2.0) / 2.0).sin();
                vehicle.velocity_kmh += (vehicle.acceleration_ms2*physical_time) * 3.6; // Velocity

                vehicle.position_m += (vehicle.velocity_kmh/3.6) * physical_time; // Position
            }
            State::BRAKING =>{ // Calculate Acceleration, then update Velocity and Position
                if vehicle.position_m>=50.0{ //If hit barrier, Stop and Critical Message
                    vehicle.position_m=50.0;   
                    vehicle.velocity_kmh=0.0;
                    vehicle.state = State::STOPPED;
                    let msg = stopped_critical(&vehicle);
                    let _ = tx.send(serde_json::to_string(&msg).unwrap());
                    continue;
                }
                if vehicle.velocity_kmh<=0.0{ //If stopped before barrier, Stop and Success Message
                    vehicle.velocity_kmh=0.0;
                    vehicle.state = State::STOPPED;
                    let msg = stopped_success(&vehicle);
                    let _ = tx.send(serde_json::to_string(&msg).unwrap());
                    continue;
                }
                let force = 196.0; // Brake Force

                vehicle.acceleration_ms2 = (-force)/vehicle.mass_kg; // Acceleration

                vehicle.velocity_kmh += (vehicle.acceleration_ms2*physical_time) * 3.6; // Velocity

                vehicle.position_m += (vehicle.velocity_kmh/3.6) * physical_time; // Position
            }
            State::STOPPED =>{ //Set all to 0
                vehicle.velocity_kmh = 0.0;
                vehicle.acceleration_ms2 = 0.0;
                vehicle.voltage_v = 0.0;
                vehicle.current_a = 0.0;
            }
        }

        // Send Data Message

        let data= Data{
            topic: Topic::data,
            payload: vehicle.clone()
        };
        let json = serde_json::to_string(&data).unwrap();
        let _ = tx.send(json);
    }
}