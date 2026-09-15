use skia_safe::{textlayout::TextAlign, Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn yys_yuanjieshenjupai(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = texts.first().map(|s| s.as_str()).unwrap_or("阴阳师，启动！");

    let func = |_: Vec<skia_safe::Image>| {
        let frame = load_image("yys_yuanjieshenjupai/0.png")?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(60, 20, 249, 95),
            text,
            20.0,
            60.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "yys_yuanjieshenjupai",
    yys_yuanjieshenjupai,
    min_images = 0,
    max_images = 0,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["阴阳师，启动！"],
    keywords = &["缘结神举牌"],
    date_created = local_date(2025, 10, 13),
    date_modified = local_date(2025, 10, 13),
}
