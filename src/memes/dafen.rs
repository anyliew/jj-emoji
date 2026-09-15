use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dafen(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { "满分" };

    let head_locs = [(46, 48), (46, 48), (31, 61), (31, 61), (34, 57)];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("dafen/{i}.png"))?;
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&png, (0, 0), None);
        if i >= 3 {
            let img = images[0].resize_exact((55, 55));
            canvas.draw_image(&img, head_locs[i - 3], None);
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(0, 27, png.width(), 527),
                text,
                20.0,
                35.0,
                text_params!(
                    font_families = &["033-SSFangTangTi"],
                    text_align = TextAlign::Center,
                    paint = new_paint(Color::from_rgb(0, 0, 0)),
                    stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 255), 3.0),
                ),
            )?;
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 8, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "dafen",
    dafen,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["满分"],
    keywords = &["打分"],
    date_created = local_date(2025, 5, 17),
    date_modified = local_date(2025, 5, 17),
}
