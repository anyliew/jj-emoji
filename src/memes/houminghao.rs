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

fn houminghao(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let func = |_: Vec<Image>| {
        let frame = load_image("houminghao/0.png")?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(31, 19, 508, 274),
            text,
            20.0,
            60.0,
            text_params!(
                font_families = &["FZShaoEr-M11S"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(81, 96, 115)),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "houminghao",
    houminghao,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["姐姐不乖哦"],
    keywords = &["侯明昊"],
    tags = MemeTags::bronya(),
    date_created = local_date(2025, 7, 11),
    date_modified = local_date(2025, 7, 11),
}
