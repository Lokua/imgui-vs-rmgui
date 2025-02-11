use eframe::egui;
use egui::Color32;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Color Picker",
        options,
        Box::new(|_cc| Ok(Box::<ColorApp>::default())),
    )
}

struct ColorApp {
    red: u8,
    green: u8,
    blue: u8,
    favorites: Vec<(u8, u8, u8)>,
}

impl Default for ColorApp {
    fn default() -> Self {
        Self {
            red: 128,
            green: 128,
            blue: 128,
            favorites: Vec::new(),
        }
    }
}

impl eframe::App for ColorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let frame = egui::Frame::NONE
                .inner_margin(0.0)
                .fill(Color32::from_rgb(self.red, self.green, self.blue));

            frame.show(ui, |ui| {
                ui.allocate_space(egui::vec2(ui.available_width(), 100.0));
            });

            ui.add_space(8.0);

            ui.add(egui::Slider::new(&mut self.red, 0..=255).text("Red"));
            ui.add(egui::Slider::new(&mut self.green, 0..=255).text("Green"));
            ui.add(egui::Slider::new(&mut self.blue, 0..=255).text("Blue"));

            let hex =
                format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue);
            ui.label(format!("Hex: {}", hex));

            ui.add_space(8.0);

            if ui.button("Add to Favorites").clicked() {
                if !self.favorites.contains(&(self.red, self.green, self.blue))
                {
                    self.favorites.push((self.red, self.green, self.blue));
                }
            }

            ui.add_space(16.0);

            ui.heading("Favorites");
            ui.horizontal_wrapped(|ui| {
                for color in &self.favorites {
                    let (r, g, b) = *color;
                    let button = egui::Button::new("")
                        .fill(Color32::from_rgb(r, g, b))
                        .min_size(egui::vec2(30.0, 30.0));
                    ui.add(button);
                }
            });
        });
    }
}
