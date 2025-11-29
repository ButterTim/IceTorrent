use crate::torrent::models::{Torrent, Info, Mode};

pub fn parse_torrent(_data: &[u8]) -> Result<(Torrent, [u8; 20]), Box<dyn std::error::Error>> {
    let torrent = Torrent {
        info: Info {
            name: "example.torrent".to_string(),
            piece_length: 16384,
            pieces: vec![],
            mode: Mode::SingleFile {
                length: 1024 * 1024 * 100,
                md5sum: None
            },
        },
        announce: "http://tracker.example.com:8080/announce".to_string(),
        announce_list: None,
        creation_date: Some(1234567890),
        comment: Some("Example torrent".to_string()),
        created_by: Some("IceTorrent".to_string()),
        encoding: Some("UTF-8".to_string()),
    };

    let info_hash = [0u8; 20];

    Ok((torrent, info_hash))
}

pub fn get_total_size(info: &Info) -> u64 {
    match &info.mode {
        Mode::SingleFile { length, .. } => *length,
        Mode::MultiFile { files } => files.iter().map(|f| f.length).sum(),
    }
}
