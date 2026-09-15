use crate::structs::structs::{Message, MessageType, PayloadMessage, PayloadVehicle, Topic};

pub fn precharge_done() -> Message{
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::success,
            content: "V = 400V precharge completed successfully".to_string()
        }
    }
}

pub fn stopped_success(vehicle: &PayloadVehicle) -> Message{
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::success,
            content: format!("Cart stopped at s = {} m", vehicle.position_m)
        }
    }
}
pub fn stopped_critical(vehicle: &PayloadVehicle) -> Message{
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::critical,
            content: format!("Cart reached mechanical stopper at s = {} m", vehicle.position_m)
        }
    }
}

pub fn boost_entered() -> Message{
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::info,
            content: "Booster section entered".to_string()
        }
    }
}

pub fn boost_success(vehicle: &PayloadVehicle) -> Message{
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::success,
            content: format!("Boost completed. Velocity: {}",vehicle.velocity_kmh)
        }
    }
}