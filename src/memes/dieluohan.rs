use skia_safe::{Color, Image, Rect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dieluohan(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |images: Vec<Image>| {
        let mut surface = new_surface((200, 250));
        let canvas = surface.canvas();
        let mut mask_surface = new_surface((180, 50));
        let mask_canvas = mask_surface.canvas();
        let paint = new_paint(Color::from_rgb(255, 255, 255));
        mask_canvas.draw_oval(Rect::from_xywh(0.0, 0.0, 180.0, 50.0), &paint);
        let mask = mask_surface.image_snapshot();
        let avatar = images[0].resize_exact((180, 50));
        let mut y = 200;
        while y >= 0 {
            let ellipse = avatar.clip_mask(&mask);
            canvas.draw_image(&ellipse, (12, y), None);
            y -= 10;
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "dieluohan",
    dieluohan,
    min_images = 1,
    max_images = 1,
    keywords = &["叠罗汉"],
    date_created = local_date(2025, 7, 5),
    date_modified = local_date(2025, 7, 5),
}
