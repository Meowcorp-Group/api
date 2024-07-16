use std::{collections::HashMap, sync::Arc};

use axum::extract::State;
use tokio::sync::Mutex;

use crate::media::nowplaying::{NowPlaying, Song};

#[derive(Debug, Clone)]
pub struct AppState {
    pub now_playing: HashMap<String, NowPlaying>,
}

impl AppState {
	pub fn new() -> Self {
		AppState {
			now_playing: HashMap::new(),
		}
	}

	// NowPlaying

	pub async fn get_now_playing(&self, username: String) -> Option<NowPlaying> {
		self.now_playing.get(&username).cloned()
	}

	pub async fn set_now_playing(&mut self, username: String, song: Song) {
		// let tmp: u64 = 0;
		self.now_playing.insert(username, NowPlaying {song});
	}

	pub async fn remove_now_playing(&mut self, username: String) {
		self.now_playing.remove(&username);
	}
}

pub type TAppState = State<Arc<Mutex<AppState>>>;