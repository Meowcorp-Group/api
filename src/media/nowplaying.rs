use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::util::state::TAppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    title: String,
    artist: String,
    album: String,
    artwork: String,
    duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSongPacket {
    username: String,
    now_playing: Song,
}

pub struct SeekPacket {
    username: String,
    seek: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlaying {
    pub song: Song,
    // pub position: u64,
}

pub async fn nowplaying_socket(ws: WebSocketUpgrade, State(state): TAppState) -> Response {
    ws.on_upgrade(move |socket| socket_handler(socket, State(state)))
}

pub async fn nowplaying_get(
    Path(username): Path<String>,
    State(state): TAppState,
) -> Json<NowPlaying> {
    dbg!(&state);
    let state = state.lock().await;
	let np = state.get_now_playing(username).await.unwrap();
    Json(np)
}

async fn socket_handler(mut socket: WebSocket, State(state): TAppState) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        // dbg!(&msg);

        if let Message::Text(text) = msg {
            dbg!(&text);
            if let Ok(np) = deserialize(&text) {
                dbg!(&np);
                let mut state = state.lock().await;
				dbg!("reached lock");
				state.set_now_playing(np.username, np.now_playing).await;
				dbg!("reached set");
				dbg!(&state);
            }
        }
    }
}

fn deserialize(msg: &String) -> Result<ChangeSongPacket, serde_json::Error> {
    serde_json::from_str(msg)
}

// async fn get_current(state: NowPlayingState) -> impl IntoResponse {
//     let state_guard = state.read().await;
//     let current_state: Vec<_> = state_guard.values().cloned().collect();
//     serde_json::to_string(&current_state).unwrap()
// }
