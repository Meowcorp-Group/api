use axum::{
    debug_handler,
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::Response,
    Json,
};
use futures::{sink::SinkExt, stream::StreamExt};
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
    now_playing: Song,
    position: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SeekPacket {
    seek: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClearPacket {
    clear: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowPlaying {
    pub song: Song,
    pub last_position: f64,
}

pub async fn nowplaying_handler(
    ws: WebSocketUpgrade,
    Path(username): Path<String>,
    State(state): State<TAppState>,
) -> Response {
    ws.on_upgrade(move |socket| nowplaying_socket(socket, username, State(state)))
}

#[debug_handler(state = TAppState)]
pub async fn nowplaying_get(
    Path(username): Path<String>,
    State(state): State<TAppState>,
) -> Json<Option<NowPlaying>> {
    let state = state.lock().await;
    Json(state.get_now_playing(username))
}

async fn nowplaying_socket(
    mut socket: WebSocket,
    username: String,
    State(state): State<TAppState>,
) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = { state.lock().await.get_now_playing_clients().subscribe() };

    {
        let state = state.lock().await;
        let now_playing = state.get_now_playing(username.clone());
        dbg!(&now_playing);
        if let Some(now_playing) = now_playing {
			state.get_now_playing_clients().send("a client connected!".to_string());
		}
    }

    //     if let Message::Text(text) = msg {
    //         // dbg!(&text);
    //         if text == "get" {
    //             socket = send_nowplaying(socket, &state, &username).await;
    //         } else if let Ok(p) = from_str::<ChangeSongPacket>(&text) {
    //             // dbg!("Passed ChangeSong");
    //             {
    //                 let mut s = state.lock().await;
    //                 s.set_now_playing(username.clone(), p.now_playing, p.position);
    //             }
    //             socket = send_nowplaying(socket, &state, &username).await;
    //         } else if let Ok(p) = from_str::<ClearPacket>(&text) {
    //             // dbg!("Passed Clear");
    //             if p.clear {
    //                 {
    //                     let mut s = state.lock().await;
    //                     s.clear_now_playing(username.clone());
    //                 }
    //                 socket = send_nowplaying(socket, &state, &username).await;
    //             }
    //         } else if let Ok(p) = from_str::<SeekPacket>(&text) {
    //             // dbg!("Passed Seek");
    //             {
    //                 let mut s = state.lock().await;
    //                 let cur_song = s.get_now_playing(username.clone()).unwrap().song;
    //                 s.set_now_playing(username.clone(), cur_song, p.seek);
    //             }
    //             socket = send_nowplaying(socket, &state, &username).await;
    //         }
    //     }
    //     // dbg!(&state);
    //}
}

async fn send_nowplaying(mut socket: WebSocket, state: &TAppState, username: &String) -> WebSocket {
    let state = state.lock().await;
    let now_playing = state.get_now_playing(username.clone());
    dbg!(&now_playing);
    if let Some(now_playing) = now_playing {}
    socket
}
