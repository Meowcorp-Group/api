use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::WebSocket;
use tokio::sync::{broadcast, Mutex};

use crate::media::nowplaying::{NowPlaying, Song};

#[derive(Debug, Clone)]
pub struct AppState {
	pub now_playing_clients: broadcast::Sender<String>,
    pub now_playing: HashMap<String, NowPlaying>,
}

impl AppState {
	pub fn new() -> Self {
		AppState {
			now_playing_clients: broadcast::channel(100).0,
			now_playing: HashMap::new(),
		}
	}

	// NowPlaying

	pub fn get_now_playing_clients(&self) -> broadcast::Sender<String> {
		self.now_playing_clients.clone()
	}

	pub fn get_now_playing(&self, username: String) -> Option<NowPlaying> {
		self.now_playing.get(&username).cloned()
	}

	pub fn set_now_playing(&mut self, username: String, song: Song, position: f64) {
		// let tmp: u64 = 0;
		self.now_playing.insert(username, NowPlaying {song, last_position: position});
	}

	pub fn clear_now_playing(&mut self, username: String) {
		self.now_playing.remove(&username);
	}
}

pub type TAppState = Arc<Mutex<AppState>>;