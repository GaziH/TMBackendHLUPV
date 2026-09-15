use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::structs::structs::{CalculateQuery, CalculateResponse, CommandRequest, Message, MessageType, PayloadMessage, PayloadVehicle, State, Topic};
use axum::extract::{Query, State as AxumState};

#[derive(Serialize,Deserialize)]
pub enum Command{
    PRECHARGE,
    START,
    BRAKE,
    RESET
}
pub async fn handle_command(
    AxumState(state): AxumState<AppState>,
    Json(command_request): Json<CommandRequest>
) -> impl IntoResponse {
    let mut vehicle = state.vehicle.lock().await;
    
    let msg = match command_request.command {
        Command::PRECHARGE => {
            precharge_command(&mut vehicle).await
        }
        Command::START => {
            match command_request.payload{
                Some(payload) => {
                    start_command(&mut vehicle, payload.mass).await // Carefully unwrap payload (if no payload, everything crashed)
                }
                None => {
                    return StatusCode::BAD_REQUEST // No payload -> 400
                }
            }
        }
        Command::BRAKE => {
            brake_command(&mut vehicle).await
        }
        Command::RESET => {
            reset_command(&mut vehicle).await
        }
    };
    
    //All return a Message struct

    let code = if msg.payload.message_type == MessageType::error{
        StatusCode::BAD_REQUEST
    }else{
        StatusCode::OK
    };
    
    if let Ok(json_string) = serde_json::to_string(&msg) {
        let _ = state.tx.send(json_string); // Send message
    }

    code //return code
}

pub async fn precharge_command(vehicle: &mut PayloadVehicle) -> Message{
    if vehicle.state!=State::IDLE || vehicle.voltage_v!=0.0{ // If idle or velocity !=0 -> 400
        return Message{
            topic: Topic::message,
            payload: PayloadMessage{
                message_type: MessageType::error,
                content: "Could not precharge, voltage>0 or not in IDLE state.".to_string()
            }
        };
    }
    vehicle.state = State::PRECHARGE; // Set next state
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::info,
            content: "Precharge started".to_string()
        }
    }
}

pub async fn start_command(vehicle: &mut PayloadVehicle, mass: f32) -> Message{
    if vehicle.state!=State::READY || vehicle.voltage_v<400.0{ // if in any state not READY or Voltage < 400V -> 400
        return Message{
            topic: Topic::message,
            payload: PayloadMessage{
                message_type: MessageType::error,
                content: "Could not start, voltage<400 or not in READY state.".to_string()
            }
        }
    };
    vehicle.state = State::RUNNING; // Set next state
    vehicle.mass_kg=mass;
    vehicle.velocity_kmh=4.0;
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::info,
            content: "Booster test started. Mass: 40 kg".to_string()
        }
    }
}

pub async fn brake_command(vehicle: &mut PayloadVehicle) -> Message{
    if vehicle.state!=State::RUNNING && vehicle.state!=State::BOOSTING{ // If not in any moving state -> 400
        return Message{
            topic: Topic::message,
            payload: PayloadMessage{
                message_type: MessageType::error,
                content: format!("Command BRAKE not allowed in State {:?}.",vehicle.state)
            }
        }
    };
    vehicle.state = State::BRAKING; // Set next state
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::info,
            content: "Braking initiated.".to_string()
        }
    }
}

pub async fn reset_command(vehicle: &mut PayloadVehicle) -> Message{
    vehicle.state = State::IDLE; // Set next state
    
    // Set all to 0
    vehicle.mass_kg=0.0;
    vehicle.acceleration_ms2=0.0;
    vehicle.velocity_kmh=0.0;
    vehicle.position_m=0.0;
    vehicle.current_a = 0.0;
    vehicle.voltage_v=0.0;
    
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::info,
            content: "System reset.".to_string()
        }
    }

}



pub async fn handle_calculate(Query(params): Query<CalculateQuery>,) -> Result<Json<CalculateResponse>, StatusCode> { //Calculate distance to brake from in order to stop given distance away from barrier.

    if params.m < 0.0 || params.d < 0.0 { // Invalid inputs -> 400
        return Err(StatusCode::BAD_REQUEST);
    }

    let v0_ms: f32 = 25.0 / 3.6;
    let braking_force: f32 = 196.0;

    let d_brake = (v0_ms.powf(2.0) * params.m) / (2.0 * braking_force);

    let s_brake = (50.0 - params.d) - d_brake;
    if s_brake < 0.0 { // Impossible result (negative) -> 400
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Json(CalculateResponse {
        braking_position_m: s_brake,
    }))
}