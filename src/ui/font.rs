use egui::*;

pub fn highlight(ui: &Ui) -> Color32 {
    if ui.style().visuals == Visuals::dark() {
        Color32::from_rgb(155, 188, 15)
    } else {
        Color32::from_rgb(15, 56, 15)
    }
}

pub fn title(ui: &Ui, s: impl Into<String>) -> RichText {
    RichText::new(s)
        .font(FontId::monospace(14.))
        .color(highlight(ui))
}

pub fn monospace(s: impl Into<String>) -> RichText {
    RichText::new(s).font(FontId::monospace(12.))
}
