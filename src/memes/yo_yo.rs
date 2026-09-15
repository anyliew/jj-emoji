use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn yo_yo(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (144, 58), (160, 57), (174, 55), (178, 42), (163, 31), (163, 27),
        (150, 24), (150, 24), (50, 25), (39, 27), (28, 28), (28, 28),
        (28, 48), (39, 52), (51, 56),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("yo_yo/{i}.png"))?;
        let img = images[0].circle().resize_exact((55, 55));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&img, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 14, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "yo_yo",
    yo_yo,
    min_images = 1,
    max_images = 1,
    keywords = &["yoyo"],
    date_created = local_date(2025, 5, 15),
    date_modified = local_date(2025, 5, 15),
}
