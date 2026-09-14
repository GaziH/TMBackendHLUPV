

pub enum State{
    IDLE,
    PRECHARGE,
    READY,
    RUNNING,
    BOOSTING,
    BRAKING,
    STOPPED
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

pub enum Topic{
    message,
    data,
}

pub enum MessageType{
    info,
    success,
    error,
    critical
}

pub struct PayloadVehicle{
    pub position_m: f32,
    pub velocity_kmh: f32,
    pub acceleration_ms2: f32,
    pub mass_kg: f32,
    pub voltage_v: f32,
    pub current_a: f32,
    pub state: State,
    //pub timestamp:
}

pub struct Data{
    topic: Topic,
    payload: PayloadVehicle,
}

pub struct PayloadMessage{
    pub(crate) message_type: MessageType,
    pub(crate) content: String,
}

pub struct Message{
    pub(crate) topic: Topic,
    pub(crate) payload: PayloadMessage,
}