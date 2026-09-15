use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn capoo_qunou(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (126, 94, 120, 120), (126, 94, 120, 120), (128, 110, 110, 100), (140, 110, 110, 100),
        (126, 94, 120, 120), (126, 94, 120, 120), (126, 94, 120, 120), (126, 94, 120, 120),
        (128, 110, 110, 100), (128, 110, 110, 100), (126, 94, 120, 120), (126, 94, 120, 120),
        (126, 94, 120, 120), (126, 94, 120, 120), (128, 110, 110, 100), (128, 110, 110, 100),
        (126, 94, 120, 120), (126, 94, 120, 120),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("capoo_qunou/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        let img = images[0].circle().resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 18, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "capoo_qunou",
    capoo_qunou,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波群殴"],
    date_created = local_date(2025, 6, 6),
    date_modified = local_date(2025, 6, 6),
}
