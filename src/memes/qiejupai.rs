use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::tags::MemeTags;
use crate::{options::NoOptions, register_meme};

fn qiejupai(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if texts.is_empty() { "男銅！" } else { &texts[0] };
    let frame = load_image("qiejupai/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(164, 10, 279, 108),
        text,
        20.0,
        60.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme! {
    "qiejupai",
    qiejupai,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["男銅！"],
    keywords = &["企鹅举牌"],
    tags = MemeTags::bronya(),
    date_created = local_date(2025, 10, 22),
    date_modified = local_date(2025, 10, 22),
}
