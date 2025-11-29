use std::net::SocketAddr;

pub struct TrackerClient {
    peer_id: String,
    client: reqwest::Client,
}

impl TrackerClient {
    pub fn new() -> Self {
        Self {
            peer_id: generate_peer_id(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn announce(
        &self,
        _tracker_url: &str,
        _info_hash: &[u8; 20],
        _downloaded: u64,
        _uploaded: u64,
        _left: u64,
    ) -> Result<Vec<SocketAddr>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}

fn generate_peer_id() -> String {
    format!("-IC0001-{}", rand::random::<u64>())
}
