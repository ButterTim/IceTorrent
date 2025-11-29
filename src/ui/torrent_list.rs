use eframe::egui;
use crate::torrent::client::TorrentClient;

pub struct TorrentList {
    pub show_details: bool,
    pub show_speeds: bool,
}

impl TorrentList {
    pub fn new() -> Self {
        Self {
            show_details: true,
            show_speeds: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, client: &TorrentClient) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let states = rt.block_on(client.get_torrent_states());

        if states.is_empty() {
            self.show_empty_state(ui);
            return;
        }

        // Статистика
        self.show_stats(ui, &states);

        egui::ScrollArea::vertical()
        .auto_shrink(false)
        .show(ui, |ui| {
            for state in states {
                self.show_torrent_card(ui, state);
            }
        });
    }

    fn show_empty_state(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(
                egui::RichText::new("🌌")
                .size(64.0)
                .color(egui::Color32::from_rgb(100, 150, 200))
            );
            ui.add_space(20.0);
            ui.heading(
                egui::RichText::new("No Active Torrents")
                .color(egui::Color32::from_rgb(200, 220, 255))
            );
            ui.label(
                egui::RichText::new("Add your first torrent to get started")
                .color(egui::Color32::from_rgb(150, 180, 220))
            );
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("📁 Use File → Add Torrent")
                .color(egui::Color32::from_rgb(120, 170, 210))
            );
        });
    }

    fn show_stats(&self, ui: &mut egui::Ui, states: &[crate::torrent::models::TorrentState]) {
        let total_downloading: u64 = states.iter().map(|s| s.downloaded).sum();
        let total_uploading: u64 = states.iter().map(|s| s.uploaded).sum();
        let _total_size: u64 = states.iter().map(|s| s.total_size).sum();
        let active_torrents = states.len();

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("📊 Overview")
                    .color(egui::Color32::from_rgb(100, 200, 255))
                    .heading()
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("Active: {}", active_torrents))
                        .color(egui::Color32::from_rgb(150, 220, 255))
                    );
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!("↓ {:.1} MB", total_downloading as f64 / 1024.0 / 1024.0))
                        .color(egui::Color32::from_rgb(100, 255, 200))
                    );
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!("↑ {:.1} MB", total_uploading as f64 / 1024.0 / 1024.0))
                        .color(egui::Color32::from_rgb(255, 200, 100))
                    );
                });
            });
        });

        ui.separator();
    }

    fn show_torrent_card(&self, ui: &mut egui::Ui, state: crate::torrent::models::TorrentState) {
        let card_frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(35, 50, 75))
        .rounding(8.0)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 90, 130)))
        .inner_margin(egui::Margin::symmetric(12.0, 8.0));

        card_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Иконка статуса
                let status_icon = match state.status {
                    crate::torrent::models::TorrentStatus::Downloading => "⬇️",
                    crate::torrent::models::TorrentStatus::Seeding => "⬆️",
                    crate::torrent::models::TorrentStatus::Paused => "⏸️",
                    crate::torrent::models::TorrentStatus::Completed => "✅",
                    crate::torrent::models::TorrentStatus::Error(_) => "❌",
                };

                ui.label(
                    egui::RichText::new(status_icon)
                    .size(24.0)
                );

                ui.vertical(|ui| {
                    // Название и прогресс
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(&state.name)
                            .color(egui::Color32::from_rgb(220, 240, 255))
                            .size(16.0)
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:.1}%", state.progress * 100.0))
                                .color(egui::Color32::from_rgb(150, 220, 255))
                                .strong()
                            );
                        });
                    });

                    // Прогресс-бар
                    let progress_color = match state.status {
                        crate::torrent::models::TorrentStatus::Downloading =>
                        egui::Color32::from_rgb(0, 150, 255),
                            crate::torrent::models::TorrentStatus::Seeding =>
                            egui::Color32::from_rgb(0, 200, 100),
                            crate::torrent::models::TorrentStatus::Completed =>
                            egui::Color32::from_rgb(100, 200, 100),
                            _ => egui::Color32::from_rgb(150, 150, 150),
                    };

                    let progress_bar = egui::ProgressBar::new(state.progress)
                    .fill(progress_color)
                    .show_percentage()
                    .desired_width(ui.available_width() - 100.0);
                    ui.add(progress_bar);

                    // Детали
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("↓ {:.1} kB/s", state.download_speed / 1024.0))
                            .color(egui::Color32::from_rgb(100, 255, 200))
                            .small()
                        );

                        ui.label(
                            egui::RichText::new(format!("↑ {:.1} kB/s", state.upload_speed / 1024.0))
                            .color(egui::Color32::from_rgb(255, 200, 100))
                            .small()
                        );

                        ui.label(
                            egui::RichText::new(format!("👥 {} peers", state.peers.len()))
                            .color(egui::Color32::from_rgb(180, 200, 255))
                            .small()
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:.1} MB / {:.1} MB",
                                                            state.downloaded as f64 / 1024.0 / 1024.0,
                                                            state.total_size as f64 / 1024.0 / 1024.0))
                                .color(egui::Color32::from_rgb(150, 180, 220))
                                .small()
                            );
                        });
                    });
                });
            });

            // Дополнительная информация
            if self.show_details {
                ui.collapsing(
                    egui::RichText::new("🔍 Details").color(egui::Color32::from_rgb(150, 200, 255)),
                              |ui| {
                                  ui.horizontal(|ui| {
                                      ui.vertical(|ui| {
                                          ui.label(
                                              egui::RichText::new("Total Size:")
                                              .color(egui::Color32::from_rgb(180, 220, 255))
                                          );
                                          ui.label(
                                              egui::RichText::new("Downloaded:")
                                              .color(egui::Color32::from_rgb(180, 220, 255))
                                          );
                                          ui.label(
                                              egui::RichText::new("Uploaded:")
                                              .color(egui::Color32::from_rgb(180, 220, 255))
                                          );
                                      });

                                      ui.vertical(|ui| {
                                          ui.label(
                                              egui::RichText::new(format!("{:.2} MB", state.total_size as f64 / 1024.0 / 1024.0))
                                              .color(egui::Color32::from_rgb(200, 230, 255))
                                          );
                                          ui.label(
                                              egui::RichText::new(format!("{:.2} MB", state.downloaded as f64 / 1024.0 / 1024.0))
                                              .color(egui::Color32::from_rgb(100, 255, 200))
                                          );
                                          ui.label(
                                              egui::RichText::new(format!("{:.2} MB", state.uploaded as f64 / 1024.0 / 1024.0))
                                              .color(egui::Color32::from_rgb(255, 200, 100))
                                          );
                                      });
                                  });
                              }
                );
            }
        });

        ui.add_space(8.0);
    }
}
