use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

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

fn payment_code(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("payment_code/0.png")?;
    let text = if texts.is_empty() { "收款码" } else { &texts[0] };
    let w = frame.width();
    let h = frame.height();

    let func = |imgs: Vec<Image>| {
        let img = imgs[0].resize_fit((360, 360), Fit::Contain);
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(60, h - 120, w - 70, h - 10),
            text,
            30.0,
            60.0,
            text_params!(
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(255, 255, 255)),
            ),
        )?;
        canvas.draw_image(&img, (120, 220), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "payment_code",
    payment_code,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["收款码"],
    keywords = &["收款码", "付款码"],
    date_created = local_date(2024, 5, 12),
    date_modified = local_date(2024, 5, 16),
}
