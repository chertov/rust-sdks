use livekit::room::{RoomEvent, RoomResult, SimulateScenario};

#[derive(Debug)]
pub enum AsyncCmd {
    RoomConnect { url: String, token: String },
    SimulateScenario { scenario: SimulateScenario },
}

#[derive(Debug)]
pub enum UiCmd {
    ConnectResult { result: RoomResult<()> },
    RoomEvent { event: RoomEvent },
}
