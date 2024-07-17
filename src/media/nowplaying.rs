use axum::{
    debug_handler,
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::util::state::TAppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    title: String,
    artist: String,
    album: String,
    artwork: String,
    duration: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeSongPacket {
    username: String,
    now_playing: Song,
	position: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SeekPacket {
    username: String,
    seek: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClearPacket {
    username: String,
    clear: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlaying {
    pub song: Song,
    pub last_position: f64,
}

pub async fn nowplaying_socket(ws: WebSocketUpgrade, State(state): State<TAppState>) -> Response {
    ws.on_upgrade(move |socket| socket_handler(socket, State(state)))
}

#[debug_handler(state = TAppState)]
pub async fn nowplaying_get(
    Path(username): Path<String>,
    State(state): State<TAppState>,
) -> Json<Option<NowPlaying>> {
    let state = state.lock().await;
    Json(state.get_now_playing(username))
}

async fn socket_handler(mut socket: WebSocket, State(state): State<TAppState>) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        // dbg!(&msg);

        if let Message::Text(text) = msg {
            dbg!(&text);
            if let Ok(p) = from_str::<ChangeSongPacket>(&text) {
                dbg!("Passed ChangeSong");
                let mut state = state.lock().await;
                state.set_now_playing(p.username, p.now_playing, p.position);
            }

            if let Ok(p) = from_str::<ClearPacket>(&text) {
                dbg!("Passed Clear");
                if p.clear {
                    let mut state = state.lock().await;
                    state.clear_now_playing(p.username);
                }
            }

			if let Ok(p) = from_str::<SeekPacket>(&text) {
				dbg!("Passed Seek");
				let mut state = state.lock().await;
				let cur_song = state.get_now_playing(p.username.clone()).unwrap().song;
				state.set_now_playing(p.username.clone(), cur_song, p.seek)
			}
        }
        dbg!(&state);
    }
}
