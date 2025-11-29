use eframe::egui;
use crate::torrent::client::TorrentClient;
use super::{TorrentList, AddTorrentDialog};

pub struct MainWindow {
    torrent_client: TorrentClient,
    torrent_list: TorrentList,
    add_dialog: Option<AddTorrentDialog>,
}

impl MainWindow {
    pub fn new(torrent_client: TorrentClient) -> Self {
        Self {
            torrent_client,
            torrent_list: TorrentList::new(),
            add_dialog: None,
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Устанавливаем визуальную тему
        self.set_visuals(ctx);

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            self.show_header(ui);
        });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_menu_bar(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.torrent_list.show(ui, &self.torrent_client);
        });

        if let Some(dialog) = &mut self.add_dialog {
            dialog.show(ctx, &self.torrent_client);
            if dialog.is_finished() {
                self.add_dialog = None;
            }
        }
    }
}

impl MainWindow {
    fn set_visuals(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // Сине-голубая цветовая схема
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 40, 60);
        style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(200, 220, 255);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 80, 120);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 120, 180);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 100, 160);

        style.visuals.extreme_bg_color = egui::Color32::from_rgb(20, 30, 45);
        style.visuals.faint_bg_color = egui::Color32::from_rgb(35, 50, 70);
        style.visuals.code_bg_color = egui::Color32::from_rgb(40, 60, 85);
        style.visuals.warn_fg_color = egui::Color32::from_rgb(255, 200, 100);
        style.visuals.error_fg_color = egui::Color32::from_rgb(255, 100, 100);
        style.visuals.window_fill = egui::Color32::from_rgb(25, 35, 50);
        style.visuals.panel_fill = egui::Color32::from_rgb(30, 45, 65);

        style.visuals.selection.bg_fill = egui::Color32::from_rgb(0, 100, 200);
        style.visuals.selection.stroke.color = egui::Color32::from_rgb(100, 180, 255);

        ctx.set_style(style);
    }

    fn show_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(
                egui::RichText::new("❄️ IceTorrent")
                .color(egui::Color32::from_rgb(100, 200, 255))
                .size(28.0)
            );
            ui.label(
                egui::RichText::new("Lightweight BitTorrent Client")
                .color(egui::Color32::from_rgb(150, 200, 230))
                .italics()
            );
            ui.add_space(5.0);
        });
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(
                egui::RichText::new("📁 File").color(egui::Color32::from_rgb(200, 220, 255)),
                           |ui| {
                               if ui.button("📥 Add Torrent").clicked() {
                                   self.add_dialog = Some(AddTorrentDialog::new());
                                   ui.close_menu();
                               }

                               ui.separator();

                               if ui.button("🚪 Exit").clicked() {
                                   ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                               }
                           }
            );

            ui.menu_button(
                egui::RichText::new("👁️ View").color(egui::Color32::from_rgb(200, 220, 255)),
                           |ui| {
                               ui.checkbox(&mut self.torrent_list.show_details, "🔍 Show Details");
                               ui.checkbox(&mut self.torrent_list.show_speeds, "📊 Show Speed Graph");
                           }
            );

            ui.menu_button(
                egui::RichText::new("⚙️ Settings").color(egui::Color32::from_rgb(200, 220, 255)),
                           |ui| {
                               if ui.button("🔧 Preferences").clicked() {
                                   // TODO: Open settings
                               }
                           }
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("v0.1.0")
                    .color(egui::Color32::from_rgb(150, 180, 220))
                    .small()
                );
            });
        });
    }
}
