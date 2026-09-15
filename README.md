# Hyperloop UPV September 2026 Training Month - Booster Test Bench Emulator (Rust Backend) - Gazi

This is Gazi's submission for the Rust Backend Task for the September 2026 Software Training Month. Built in Rust, it emulates the physics and state machine of the booster cart, exposing a dual-port API for real-time frontend telemetry and command control.

## 🚀 Getting Started

### Prerequisites
* [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Installation & Execution
1. Clone this repository to your local machine.
2. Navigate to the root directory of the project.
3. Start the server using Cargo:
   ```bash
   cargo run
   ```
4. The server will initialize two simultaneous local services:
   * **HTTP API (Commands):** `http://localhost:8001`
   * **WebSocket (Telemetry):** `ws://localhost:5001/backend/stream`

---

## 📡 API Reference

### 1. HTTP Commands (Port 8001)

#### `POST /api/command`
Sends state machine transitions to the emulator. If a command is invalid for the current state, a `400 Bad Request` is returned.

**Standard Commands (`PRECHARGE`, `BRAKE`, `RESET`)**
```json
{
  "command": "PRECHARGE"
}
```

**START Command** (Requires the vehicle mass in kilograms)
```json
{
  "command": "START",
  "payload": {
    "mass": 40.0
  }
}
```

#### `GET /api/calculate`
Calculates the optimal track position to trigger the brakes to stop at a the given distance to the end of the track. 
* **Query Parameters:** `?m=40&d=5` (`m` is mass in kg, `d` is distance to stop from end of 50m track in metres).

**Success Response (200 OK):**
```json
{
  "braking_position_m": 40.07905
}
```

---

### 2. WebSocket Telemetry (Port 5001)

Clients can connect to `ws://localhost:5001/backend/stream` to receive live simulation data. The server emits two distinct JSON formats:

**Continuous Telemetry (`topic: data`)**
Broadcasted at 4 Hz.
```json
{
    "topic": "data",
    "payload": {
        "position_m": 3.5244076,
        "velocity_kmh": 21.509148,
        "acceleration_ms2": 12.743592,
        "mass_kg": 40.0,
        "voltage_v": 400.0,
        "current_a": 187.56537,
        "state": "BOOSTING",
        "timestamp": "2026-09-15T17:57:31.349899500+00:00"
    }
}
```

**Event Alerts (`topic: message`)**
Broadcasted dynamically during state transitions or rule validations. Types include `info`, `success`, `error`, and `critical`.
```json
{
    "topic": "message",
    "payload": {
        "message_type": "success",
        "content": "Boost completed. Velocity: 25"
    }
}
```
