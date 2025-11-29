pub struct Handshake {
    pub protocol: String,
    pub info_hash: [u8; 20],
    pub peer_id: String,
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: String) -> Self {
        Self {
            protocol: "BitTorrent protocol".to_string(),
            info_hash,
            peer_id,
        }
    }
}

pub enum Message {
    KeepAlive,
    Choke,
    Unchoke,
    Interested,
    NotInterested,
}
