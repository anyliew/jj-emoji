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

fn sibalu(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!("{}", texts[0]);
    let head_locs = [
        (-8, 118), (-8, 118), (-8, 118), (-8, 118), (-8, 118), (-8, 118),
        (-8, 118), (-8, 118), (-8, 118), (-8, 118), (-8, 118), (-5, 115),
        (-5, 112), (10, 112), (78, 108), (94, 107), (94, 107), (94, 107),
        (94, 107), (95, 109), (95, 109), (95, 109), (95, 109),
    ];
    let rotate_num = [
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, -6, -6, 4, 4, 4, 4, 4, 4, 4, 4,
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("sibalu/{i}.png"))?;
        let r = images[0].resize_exact((42, 93));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        if i <= 22 {
            let rotated = r.rotate_crop(-(rotate_num[i] as f32));
            let (x, y) = head_locs[i];
            canvas.draw_image(&rotated, (x, y), None);
        }
        canvas.draw_image(&bg, (0, 0), None);
        if i <= 22 {
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(0, 182, bg.width(), 212),
                &text,
                20.0,
                20.0,
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
        GifInfo { frame_num: 31, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "sibalu",
    sibalu,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["给你看个傻子"],
    keywords = &["486"],
    date_created = local_date(2025, 7, 9),
    date_modified = local_date(2025, 7, 9),
}
