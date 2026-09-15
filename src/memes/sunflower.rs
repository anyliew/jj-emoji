use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn sunflower(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (91, 121), (91, 107), (91, 85), (91, 98), (91, 85), (100, 93),
        (100, 108), (100, 114), (105, 128), (109, 130),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("sunflower/{i}.png"))?;
        let img = images[0].circle().resize_exact((90, 90));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&img, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 10, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "sunflower",
    sunflower,
    min_images = 1,
    max_images = 1,
    keywords = &["太阳花"],
    date_created = local_date(2025, 5, 14),
    date_modified = local_date(2025, 5, 14),
}
