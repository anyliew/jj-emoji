use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn chuai(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (14, 220), (14, 220), (14, 220), (14, 220), (14, 220), (14, 220), (14, 220), (14, 220),
        (14, 220), (14, 220), (14, 220), (14, 220), (14, 220), (14, 220), (14, 220), (14, 220),
        (12, 220), (10, 220), (5, 220), (10, 220), (14, 220), (14, 220),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("chuai/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&bg, (0, 0), None);
        let img = images[0].circle().resize_exact((80, 80));
        canvas.draw_image(&img, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 22, duration: 0.04 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "chuai",
    chuai,
    min_images = 1,
    max_images = 1,
    keywords = &["踹"],
    date_created = local_date(2025, 7, 31),
    date_modified = local_date(2025, 7, 31),
}
