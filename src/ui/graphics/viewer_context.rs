use egui::*;

#[derive(Default)]
pub struct ViewerContext<'a> {
    pub image: Option<Image<'a>>,
    pub texture: Option<TextureHandle>,
}
