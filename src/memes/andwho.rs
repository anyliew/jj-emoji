use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn andwho(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { "原神" };

    let func = |images: Vec<Image>| {
        let background = load_image("andwho/0.png")?;
        let img = images[0].square().resize_exact((300, 300));
        let mut surface = new_surface(background.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (430, 272), None);
        canvas.draw_image(&background, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 444, 252, 615),
            text,
            24.0,
            72.0,
            text_params!(
                font_families = &["GlowSansSC-Normal-Heavy"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(0, 0, 0), 1.0),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "andwho",
    andwho,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["原神"],
    keywords = &["今天和谁过"],
    date_created = local_date(2025, 8, 29),
    date_modified = local_date(2025, 8, 29),
}
