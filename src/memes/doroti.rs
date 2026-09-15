use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doroti(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (33, 124, 70, 62), (30, 122, 70, 62), (30, 122, 70, 62), (30, 122, 70, 62), (30, 122, 70, 62),
        (30, 122, 70, 62), (30, 122, 70, 62), (30, 122, 70, 62), (30, 122, 70, 62), (30, 122, 70, 62),
    ];
    let tow_locs = [
        (28, 132, 62, 55), (24, 111, 62, 55), (-12, 64, 62, 55), (-51, 43, 62, 55),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("doroti/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        if i < locs.len() {
            let (x1, y1, w1, h1) = locs[i];
            canvas.draw_image(&images[0].circle().resize_exact((w1, h1)), (x1, y1), None);
        }
        if (10..=14).contains(&i) && i - 10 < tow_locs.len() {
            let (x2, y2, w2, h2) = tow_locs[i - 10];
            canvas.draw_image(&images[0].circle().resize_exact((w2, h2)), (x2, y2), None);
        }
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 19, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "doroti",
    doroti,
    min_images = 1,
    max_images = 1,
    keywords = &["doro踢"],
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 5, 27),
}
