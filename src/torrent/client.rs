use crate::torrent::models::TorrentState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct TorrentClient {
    torrents: Arc<RwLock<HashMap<String, TorrentState>>>,
}

impl TorrentClient {
    pub fn new() -> Self {
        Self {
            torrents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_torrent(&self, file_path: String, _download_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let state = TorrentState {
            name: file_path.clone(),
            ..Default::default()
        };

        self.torrents.write().await.insert(file_path, state);
        Ok(())
    }

    pub async fn get_torrent_states(&self) -> Vec<TorrentState> {
        let torrents = self.torrents.read().await;
        torrents.values().cloned().collect()
    }
}
