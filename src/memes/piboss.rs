use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn piboss(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (96, 104, 90, 190),
        (79, 118, 110, 170),
        (91, 97, 100, 190),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("piboss/{i}.png"))?;
        let (x, y, w, h) = locs[i];
        let r = images[0].circle().resize_exact((w, h)).rotate_crop(10.0);
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&r, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 3, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "piboss",
    piboss,
    min_images = 1,
    max_images = 1,
    keywords = &["痞老板"],
    date_created = local_date(2025, 5, 30),
    date_modified = local_date(2025, 5, 30),
}
