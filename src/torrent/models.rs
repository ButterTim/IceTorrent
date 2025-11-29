#[derive(Debug, Clone)]
pub struct TorrentFile {
    pub path: Vec<String>,
    pub length: u64,
    pub md5sum: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Info {
    pub name: String,
    pub piece_length: u64,
    pub pieces: Vec<u8>,
    pub mode: Mode,
}

#[derive(Debug, Clone)]
pub enum Mode {
    SingleFile { length: u64, md5sum: Option<String> },
    MultiFile { files: Vec<TorrentFile> },
}

#[derive(Debug, Clone)]
pub struct Torrent {
    pub info: Info,
    pub announce: String,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<u64>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TorrentState {
    pub info_hash: [u8; 20],
    pub name: String,
    pub total_size: u64,
    pub downloaded: u64,
    pub uploaded: u64,
    pub peers: Vec<Peer>,
    pub status: TorrentStatus,
    pub download_speed: f64,
    pub upload_speed: f64,
    pub progress: f32,
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub ip: String,
    pub port: u16,
    pub client: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TorrentStatus {
    Downloading,
    Seeding,
    Paused,
    Completed,
    Error(String),
}

impl Default for TorrentState {
    fn default() -> Self {
        Self {
            info_hash: [0; 20],
            name: "Example Torrent".to_string(),
            total_size: 1024 * 1024 * 100,
            downloaded: 1024 * 1024 * 30,
            uploaded: 1024 * 1024 * 5,
            peers: vec![
                Peer { ip: "192.168.1.1".to_string(), port: 6881, client: "qBittorrent".to_string() },
                Peer { ip: "10.0.0.2".to_string(), port: 6881, client: "Transmission".to_string() },
            ],
            status: TorrentStatus::Downloading,
            download_speed: 1024.0 * 512.0,
            upload_speed: 1024.0 * 64.0,
            progress: 0.3,
        }
    }
}
