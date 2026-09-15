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

fn kurogames_chun_holdsign(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let func = |_: Vec<Image>| {
        let frame = load_image("kurogames_chun_holdsign/0.png")?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(53, 286, 312, 458),
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
    "kurogames_chun_holdsign",
    kurogames_chun_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["傻？我不傻"],
    keywords = &["椿举牌"],
    tags = MemeTags::bronya(),
    date_created = local_date(2025, 6, 30),
    date_modified = local_date(2025, 6, 30),
}
