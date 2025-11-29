use eframe::egui;
use crate::torrent::client::TorrentClient;

pub struct AddTorrentDialog {
    path: String,
    download_path: String,
    is_finished: bool,
}

impl AddTorrentDialog {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            download_path: "downloads".to_string(),
            is_finished: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, client: &TorrentClient) {
        egui::Window::new("📥 Add Torrent")
        .collapsible(false)
        .resizable(false)
        .frame(egui::Frame::window(&ctx.style()).fill(egui::Color32::from_rgb(35, 50, 75)))
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading(
                    egui::RichText::new("Add New Torrent")
                    .color(egui::Color32::from_rgb(200, 220, 255))
                );

                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new("Torrent file:")
                    .color(egui::Color32::from_rgb(180, 210, 240))
                );

                ui.horizontal(|ui| {
                    let text_edit = egui::TextEdit::singleline(&mut self.path)
                    .hint_text("Path to .torrent file or magnet link...")
                    .desired_width(300.0)
                    .frame(true);
                    ui.add(text_edit);

                    if ui.button("📁 Browse").clicked() {
                        self.path = "/path/to/example.torrent".to_string();
                    }
                });

                ui.add_space(8.0);

                ui.label(
                    egui::RichText::new("Download location:")
                    .color(egui::Color32::from_rgb(180, 210, 240))
                );

                let download_edit = egui::TextEdit::singleline(&mut self.download_path)
                .hint_text("Where to save downloaded files...")
                .desired_width(300.0)
                .frame(true);
                ui.add(download_edit);

                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    if ui.button(
                        egui::RichText::new("❌ Cancel")
                        .color(egui::Color32::from_rgb(255, 100, 100))
                    ).clicked() {
                        self.is_finished = true;
                    }

                    ui.add_space(10.0);

                    let add_button = egui::Button::new(
                        egui::RichText::new("✅ Add Torrent")
                        .color(egui::Color32::from_rgb(255, 255, 255))
                    ).fill(egui::Color32::from_rgb(0, 150, 255));

                    if ui.add(add_button).clicked() {
                        let client = client.clone();
                        let path = self.path.clone();
                        let download_path = self.download_path.clone();

                        tokio::spawn(async move {
                            if let Err(e) = client.add_torrent(path, download_path).await {
                                eprintln!("Failed to add torrent: {}", e);
                            }
                        });

                        self.is_finished = true;
                    }
                });
            });
        });
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
}
