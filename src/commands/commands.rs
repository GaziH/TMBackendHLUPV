use crate::structs::structs::{Message, MessageType, PayloadMessage, PayloadVehicle, State, Topic};

pub enum Command{
    PRECHARGE,
    START,
    BRAKE,
    RESET
}

async fn precharge_command(vehicle: &mut PayloadVehicle) -> Message{
    if (vehicle.state!=State::IDLE || vehicle.voltage_v!=0.0){
        return Message{
            topic: Topic::message,
            payload: PayloadMessage{
                message_type: MessageType::error,
                content: "Could not precharge, voltage>0 or not in IDLE state.".to_string()
            }
        };
    }
    vehicle.state = State::PRECHARGE;
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::info,
            content: "Precharge started".to_string()
        }
    }
}

async fn precharge_done(vehicle: &mut PayloadVehicle) -> Message{
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::success,
            content: "V = 400V precharge completed successfully".to_string()
        }
    }
}

async fn start_command(vehicle: &mut PayloadVehicle) -> Message{
    if(vehicle.state!=State::READY || vehicle.voltage_v<400.0){
        return Message{
            topic: Topic::message,
            payload: PayloadMessage{
                message_type: MessageType::error,
                content: "Could not start, voltage<400 or not in READY state.".to_string()
            }
        }
    };
    vehicle.state = State::RUNNING;
    vehicle.velocity_kmh=4.0;
    Message{
        topic: Topic::message,
        payload: PayloadMessage{
            message_type: MessageType::info,
            content: "Booster test started. Mass: 40 kg".to_string()
        }
    }
}