use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    text_params,
    tools::{color_from_str, load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn daxiaojiejiadao(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { "通通闪开，大小姐驾到！" };

    let locs = [
        (144, 18, 27, 32), (144, 15, 27, 32), (141, 12, 27, 32), (139, 10, 27, 37), (131, 10, 31, 39),
        (146, 10, 31, 39), (146, 0, 33, 46), (108, -8, 34, 43), (101, -7, 34, 43), (102, -8, 39, 45),
        (96, 0, 41, 49), (113, 18, 37, 50), (118, 23, 37, 50), (161, 31, 36, 43), (153, 28, 36, 43),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("daxiaojiejiadao/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        let img = images[0].circle().resize_exact((100, 100)).resize_exact((w, h));
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 200, bg.width(), 245),
            text,
            18.0,
            24.0,
            text_params!(
                font_families = &["System"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(color_from_str("#d6e5fc"), 0.2),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 15, duration: 0.12 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "daxiaojiejiadao",
    daxiaojiejiadao,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["通通闪开，大小姐驾到！"],
    keywords = &["大小姐驾到"],
    date_created = local_date(2025, 8, 13),
    date_modified = local_date(2025, 8, 13),
}
