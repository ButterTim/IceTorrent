mod ui;
mod torrent;
mod network;

use eframe::{egui, NativeOptions};
use ui::MainWindow;
use torrent::client::TorrentClient;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_title("IceTorrent"),
        ..Default::default()
    };

    let torrent_client = TorrentClient::new();
    let main_window = MainWindow::new(torrent_client);

    eframe::run_native(
        "IceTorrent",
        options,
        Box::new(|_cc| Box::new(main_window)),
    )
}
