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

fn penshui(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!("出来喷水，大水逼{}", texts[0]);
    let locs = [
        (66, 336), (56, 285), (51, 232), (51, 232), (51, 232),
        (51, 232), (51, 232), (51, 232), (51, 232),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("penshui/{i}.png"))?;
        let r = images[0].circle().resize_exact((85, 85));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        if (8..=16).contains(&i) {
            let (x, y) = locs[i - 8];
            canvas.draw_image(&r, (x, y), None);
        }
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 12, bg.width(), 512),
            &text,
            40.0,
            40.0,
            text_params!(
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(0, 0, 255), 0.2),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 22, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "penshui",
    penshui,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["龙王"],
    keywords = &["喷水"],
    date_created = local_date(2025, 5, 20),
    date_modified = local_date(2025, 5, 20),
}
