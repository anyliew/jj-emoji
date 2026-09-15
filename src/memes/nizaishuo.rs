use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn nizaishuo(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("nizaishuo/0.png")?;
    let text = if texts.is_empty() { "你闭嘴！" } else { &texts[0] };

    let func = |imgs: Vec<Image>| {
        let img = imgs[0].circle().resize_fit((60, 50), Fit::Contain);
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_text(
            (10, 12),
            text,
            20.0,
            text_params!(
                font_families = &["GlowSansSC-Normal-Heavy"],
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        );
        canvas.draw_image(&img, (88, 14), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "nizaishuo",
    nizaishuo,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["你闭嘴！"],
    keywords = &["你再说", "你闭嘴"],
    date_created = local_date(2025, 6, 16),
    date_modified = local_date(2025, 6, 19),
}
