use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, WebSocketUpgrade,
    },
    response::{IntoResponse, Response}, Extension, Json,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlaying {
    title: String,
    artist: String,
    album: String,
    artwork: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlayingPacket {
    username: String,
    now_playing: NowPlaying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlayingUser {
    now_playing: Option<NowPlaying>,
}

#[derive(Debug, Clone)]
pub struct NowPlayingState {
    users: Arc<Mutex<HashMap<String, NowPlayingUser>>>,
}

impl NowPlayingState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn set_now_playing(&self, username: String, now_playing: NowPlaying) {
        let mut users = self.users.lock().await;
        let user = users
            .entry(username)
            .or_insert(NowPlayingUser { now_playing: None });
        user.now_playing = Some(now_playing);
    }

	pub async fn get_now_playing(&self, username: String) -> Option<NowPlaying> {
		let users = self.users.lock().await;
		users.get(&username).map(|u| u.now_playing.clone()).flatten()
	}
}

pub async fn nowplaying_socket(ws: WebSocketUpgrade, state: Extension<Arc<NowPlayingState>>) -> Response {
    ws.on_upgrade(|socket| socket_handler(socket, state))
}

pub async fn nowplaying_get(Path(username): Path<String>, state: Extension<Arc<NowPlayingState>>) -> Json<Option<NowPlaying>> {
	let np = state.get_now_playing(username).await;
	Json(np)
}

async fn socket_handler(mut socket: WebSocket, state: Extension<Arc<NowPlayingState>>) {
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
				state.set_now_playing(np.username, np.now_playing).await;
            }
        }
    }
}

fn deserialize(msg: &String) -> Result<NowPlayingPacket, serde_json::Error> {
    serde_json::from_str(msg)
}

// async fn get_current(state: NowPlayingState) -> impl IntoResponse {
//     let state_guard = state.read().await;
//     let current_state: Vec<_> = state_guard.values().cloned().collect();
//     serde_json::to_string(&current_state).unwrap()
// }
