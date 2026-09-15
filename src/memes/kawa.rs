use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::tags::MemeTags;
use crate::{options::NoOptions, register_meme};

fn kawa(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let func = |_: Vec<Image>| {
        let frame = load_image("kawa/0.png")?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        for (dx, dy) in [
            (-2, -2), (-2, 0), (-2, 2), (0, -2), (0, 2), (2, -2), (2, 0), (2, 2),
        ] {
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(70 + dx, 120 + dy, 412 + dx, 231 + dy),
                text,
                20.0,
                255.0,
                text_params!(
                    font_families = &["033-SSFangTangTi"],
                    text_align = TextAlign::Center,
                    paint = new_paint(Color::from_rgb(0, 0, 0)),
                ),
            )?;
        }
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(70, 120, 412, 231),
            text,
            20.0,
            250.0,
            text_params!(
                font_families = &["033-SSFangTangTi"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(255, 180, 221)),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "kawa",
    kawa,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["卡哇伊"],
    keywords = &["kawa", "卡哇伊"],
    tags = MemeTags::bronya(),
    date_created = local_date(2025, 7, 8),
    date_modified = local_date(2025, 7, 8),
}
