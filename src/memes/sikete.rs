use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn sikete(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = "正义斯科特";
    let text = if texts.is_empty() { "那我的屁股怎么办" } else { &texts[0] };
    let img = images[0].image.circle().resize_exact((260, 230));
    let frame_bg = load_image("sikete/0.png")?;
    let mut surface = new_surface(frame_bg.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(&img, (135, 31), None);
    canvas.draw_image(&frame_bg, (0, 0), None);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(136, 382, 360, 437),
        text,
        18.0,
        18.0,
        text_params!(
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(255, 255, 255)),
        ),
    )?;
    let cx = frame_bg.width() / 2 - 80;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(cx - 200, 348, cx + 120, 388),
        name,
        18.0,
        18.0,
        text_params!(
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(232, 214, 173)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme! {
    "sikete",
    sikete,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["那我的屁股怎么办"],
    keywords = &["斯科特"],
    date_created = local_date(2025, 7, 13),
    date_modified = local_date(2025, 7, 13),
}
