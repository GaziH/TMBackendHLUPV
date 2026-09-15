use std::sync::Arc;
use axum::extract::{State as AxumState, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::{ Router};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::CorsLayer;
use crate::commands::commands::{handle_calculate, handle_command};
use crate::commands::physics::tick_physics;
use crate::structs::structs::{ PayloadVehicle, State};

pub mod structs;
pub mod commands;
pub mod messages;

#[derive(Clone)]
pub struct AppState{
    pub vehicle: Arc<Mutex<PayloadVehicle>>,
    pub tx: broadcast::Sender<String>
}
#[tokio::main]
async fn main() {

    // Create Default Vehicle
    let vehicle = Arc::new(Mutex::new(PayloadVehicle{
        position_m: 0.0,
        velocity_kmh: 0.0,
        acceleration_ms2: 0.0,
        mass_kg: 0.0,
        voltage_v: 0.0,
        current_a: 0.0,
        state: State::IDLE,
        timestamp: chrono::Utc::now().to_rfc3339()
    }));

    // Create Broadcast Channel, tx send, rx recieve
    let (tx,  _rx) = broadcast::channel::<String>(100);

    // Create State of the app
    let app_state = AppState{
        vehicle: vehicle.clone(),
        tx: tx.clone()
    };

    tokio::spawn(tick_physics(vehicle.clone(),tx.clone()));

    //Websocket to send logs
    let ws_app = Router::new()
        .route("/backend/stream", get(ws_handler))
        .with_state(app_state.clone())
        .layer(CorsLayer::permissive());

    let ws_listener = TcpListener::bind("0.0.0.0:5001").await.unwrap();

    tokio::spawn(async move{
        println!("Starting websocket server");
        axum::serve(ws_listener, ws_app).await.unwrap()
    });

    //Http TCP to recieve commands reliably
    let http_app = Router::new()
        .route("/api/command",post(handle_command))
        .route("/api/calculate",get(handle_calculate))
        .with_state(app_state.clone())
        .layer(CorsLayer::permissive());

    let http_listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();

    println!("HTTP API running on http://localhost:8001");
    axum::serve(http_listener, http_app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    AxumState(state): AxumState<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    // Sub to broadcast channel
    let mut rx = state.tx.subscribe();

    // Loop and wait for messages
    while let Ok(msg) = rx.recv().await {
        // Send message on websocket
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}