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

fn myplay(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = "智商-1";
    let text = if texts.is_empty() { "笨死了" } else { &texts[0] };
    let head_locs = [
        (48, 208), (48, 211), (48, 274), (48, 264), (48, 242), (48, 232),
        (48, 221), (48, 209), (48, 208), (48, 272), (48, 245), (48, 240),
        (48, 225), (48, 216), (48, 208), (48, 205),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("myplay/{i}.png"))?;
        let head = images[0].resize_exact((100, 100)).circle();
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let (x, y) = head_locs[i];
        canvas.draw_image(&head, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 265, bg.width(), 765),
            text,
            35.0,
            35.0,
            text_params!(
                font_families = &["033-SSFangTangTi"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 255), 3.0),
            ),
        )?;
        if i < 4 {
            let ny = 172 - i as i32 * 40;
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(0, ny - 30, 180, ny + 30),
                name,
                18.0,
                18.0,
                text_params!(
                    text_align = TextAlign::Center,
                    paint = new_paint(Color::from_rgb(27, 27, 27)),
                ),
            )?;
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 16, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "myplay",
    myplay,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["笨死了"],
    keywords = &["我敲"],
    date_created = local_date(2025, 5, 17),
    date_modified = local_date(2025, 5, 17),
}
