use eframe::egui::{Context, TextureHandle, TextureOptions};
use image::GenericImageView;

pub fn load_icon(ctx: &Context, path: &str) -> Option<TextureHandle> {
    let image = image::open(path).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    Some(ctx.load_texture(
        path,
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        TextureOptions::default(),
    ))
}
