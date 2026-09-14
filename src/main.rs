use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use crate::structs::structs::{PayloadVehicle, State};

pub mod structs;
pub mod commands;

fn main() {
    println!("Hello, world!");
}

async fn tick_physics(vehicle: Arc<Mutex<PayloadVehicle>>) {
    let mut ticker = interval(Duration::from_millis(250));
    let physical_time = 0.05;

    loop{
        ticker.tick().await;
        let mut vehicle = vehicle.lock().await;

        match vehicle.state {
            State::IDLE => {

            }
            State::PRECHARGE => {
                vehicle.voltage_v+=25.0;
                if(vehicle.voltage_v >=400.0){
                    vehicle.state = State::READY;
                }
            }
            State::READY => {

            }
            State::RUNNING =>{ // Calculate Position with constant Velocity
                if(vehicle.position_m>=2.0 && vehicle.position_m<=4.0){ // If in Booster zone
                    vehicle.state = State::BOOSTING;
                    continue;
                }
                if(vehicle.position_m>=50.0){ // If hits barrier
                    vehicle.state = State::STOPPED;
                    continue;
                }
                vehicle.position_m += (vehicle.velocity_kmh/3.6) * physical_time; // Position


            }
            State::BOOSTING => { // Calculate Acceleration, then update Velocity and Position
                if(vehicle.position_m>=4.0){
                    vehicle.velocity_kmh=25.0;
                    vehicle.state = State::RUNNING;
                    continue;
                }
                let target_velocity_ms: f32 = (25.0/3.6);

                vehicle.acceleration_ms2=( target_velocity_ms.powf(2.0) * (vehicle.velocity_kmh/3.6).powf(2.0) ) // Acceleration
                                                    / (2.0*(4.0-vehicle.position_m) );

                let force = vehicle.acceleration_ms2*vehicle.mass_kg;

                vehicle.velocity_kmh += (vehicle.acceleration_ms2*physical_time) * 3.6; // Velocity

                vehicle.position_m += (vehicle.velocity_kmh/3.6) * physical_time; // Position
            }
            State::BRAKING =>{

            }
            State::STOPPED =>{

            }
        }
    }
}
