use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::tags::MemeTags;
use crate::{options::NoOptions, register_meme};

fn sayguaihua(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!("{}", texts[0]);
    let fg = load_image("sayguaihua/0.png")?;
    let (w_fg, h) = (fg.width(), fg.height());

    let func = |i: usize, _: Vec<Image>| {
        let mut surface = new_surface((w_fg, h));
        let canvas = surface.canvas();
        canvas.clear(Color::from_rgb(239, 232, 213));
        let x = w_fg - i as i32 * 15;
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(x, 33, x + 590, 113),
            &text,
            25.0,
            25.0,
            text_params!(
                text_align = TextAlign::Left,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 255), 1.0),
            ),
        )?;
        canvas.draw_image(&fg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 30, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "sayguaihua",
    sayguaihua,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["你好会上班哦~"],
    keywords = &["说怪话", "阴阳大师"],
    tags = MemeTags::bronya(),
    date_created = local_date(2025, 9, 29),
    date_modified = local_date(2025, 9, 29),
}
