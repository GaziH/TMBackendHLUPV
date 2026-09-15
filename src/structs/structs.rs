use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum State{
    IDLE,
    PRECHARGE,
    READY,
    RUNNING,
    BOOSTING,
    BRAKING,
    STOPPED
}

#[derive(Serialize, Deserialize)]
pub enum Topic{
    message,
    data,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum MessageType{
    info,
    success,
    error,
    critical
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PayloadVehicle{
    pub position_m: f32,
    pub velocity_kmh: f32,
    pub acceleration_ms2: f32,
    pub mass_kg: f32,
    pub voltage_v: f32,
    pub current_a: f32,
    pub state: State,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data{
    pub(crate) topic: Topic,
    pub(crate) payload: PayloadVehicle,
}

#[derive(Serialize, Deserialize)]
pub struct PayloadMessage{
    pub(crate) message_type: MessageType,
    pub(crate) content: String,
}
#[derive(Serialize, Deserialize)]
pub struct Message{
    pub(crate) topic: Topic,
    pub(crate) payload: PayloadMessage,
}





// #[derive(Deserialize)]
// pub struct CalculateQuery {
//     pub m: f32,
//     pub d: f32,
// }

// 2. Define the exact JSON structure the frontend expects back
#[derive(Serialize)]
pub struct CalculateResponse {
    pub(crate) braking_position_m: f32,
}