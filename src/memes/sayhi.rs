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

fn sayhi(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!("打个招呼吧，{}", texts[0]);
    let locs = [
        (27, 95), (28, 95), (22, 102), (17, 100), (16, 101), (17, 101),
        (20, 108), (43, 115), (42, 99), (42, 99), (42, 99), (42, 99),
        (42, 99), (42, 99), (42, 99), (40, 99),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("sayhi/{i}.png"))?;
        let (x, y) = locs[i];
        let r = images[0].resize_exact((80, 80));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&r, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 306, bg.width(), 406),
            &text,
            25.0,
            25.0,
            text_params!(
                font_families = &["033-SSFangTangTi"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 255), 3.0),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 16, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "sayhi",
    sayhi,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["小奶龙"],
    keywords = &["打招呼"],
    date_created = local_date(2025, 6, 25),
    date_modified = local_date(2025, 6, 25),
}
