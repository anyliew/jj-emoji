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

fn buyaolian(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { "我就是不要脸\n 你来撕我啊" };

    let one_locs = [
        (130, 179, 93, 80), (130, 179, 93, 80), (137, 202, 94, 96), (152, 248, 95, 100),
        (188, 313, 69, 74),
    ];
    let tow_locs = [
        (137, 85, 90, 82), (139, 111, 93, 80), (138, 172, 90, 91), (138, 172, 90, 91),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("buyaolian/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let img = images[0].square();
        if i < 5 {
            let (x1, y1, w1, h1) = one_locs[i];
            canvas.draw_image(&img.resize_exact((w1, h1)), (x1, y1), None);
        }
        if (1..=4).contains(&i) {
            let (x2, y2, w2, h2) = tow_locs[i - 1];
            canvas.draw_image(&img.resize_exact((w2, h2)), (x2, y2), None);
        }
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 18, bg.width(), 103),
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
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 6, duration: 0.1 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "buyaolian",
    buyaolian,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我就是不要脸\n 你来撕我啊"],
    keywords = &["不要脸", "撕脸"],
    date_created = local_date(2025, 5, 24),
    date_modified = local_date(2025, 5, 24),
}
