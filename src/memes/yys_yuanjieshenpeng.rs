use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn yys_yuanjieshenpeng(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = texts.first().map(|s| s.as_str()).unwrap_or("大宝贝！");

    let func = |images: Vec<Image>| {
        let frame = load_image("yys_yuanjieshenpeng/0.png")?;
        let img = images[0].circle().resize_fit((160, 160), Fit::Contain);
        let mut text2image = Text2Image::from_text(
            text,
            60.0,
            text_params!(
                font_families = &["GlowSansSC-Normal-Heavy"],
                paint = new_paint(Color::from_rgb(255, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 255), 2.0),
            ),
        );
        text2image.layout(500.0);
        if text2image.height() > 200.0 {
            return Err(Error::TextOverLength(text.to_string()));
        }
        let text_w = text2image.longest_line().ceil() as i32;
        let x = (frame.width() - text_w) / 2;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        text2image.draw_on_canvas(&canvas, (x, 11));
        canvas.draw_image(&img, (158, 119), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "yys_yuanjieshenpeng",
    yys_yuanjieshenpeng,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["大宝贝！"],
    keywords = &["缘结神举", "缘结神捧"],
    date_created = local_date(2025, 10, 13),
    date_modified = local_date(2025, 10, 13),
}
