use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn police_car(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (81, 53), (81, 51), (81, 49), (81, 51), (81, 53), (81, 51),
        (81, 49), (81, 51), (81, 53), (81, 51), (81, 49), (81, 51),
        (81, 53), (81, 51), (81, 49), (81, 51), (81, 53), (81, 51),
        (81, 49), (81, 51), (81, 53), (81, 51), (81, 49), (81, 51),
        (81, 53), (81, 51), (81, 49), (81, 51), (81, 53), (81, 51),
        (81, 49), (81, 51), (81, 53), (81, 51), (81, 49), (81, 51),
        (81, 53), (81, 51), (81, 49), (81, 51), (81, 53), (81, 51),
        (81, 49), (81, 51), (81, 53), (81, 51),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("police_car/{i}.png"))?;
        let r = images[0].circle().resize_exact((36, 36));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&bg, (0, 0), None);
        let (x, y) = locs[i];
        canvas.draw_image(&r, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 45, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "police_car",
    police_car,
    min_images = 1,
    max_images = 1,
    keywords = &["警车"],
    date_created = local_date(2025, 5, 13),
    date_modified = local_date(2025, 5, 13),
}
