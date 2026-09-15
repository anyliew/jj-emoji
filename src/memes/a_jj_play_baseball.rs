use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn a_jj_play_baseball(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (173, 87), (180, 87), (188, 87), (196, 87), (204, 87), (211, 87), (219, 87), (227, 87),
        (235, 87), (217, 87), (198, 87), (179, 87), (160, 87), (142, 87), (123, 87), (104, 87),
        (85, 87), (99, 87), (114, 87), (129, 87), (143, 87), (158, 87), (173, 87),
    ];
    let angles = [
        90, 180, 270, 360, 450, 540, 630, 720, 810, 720, 630, 540, 450, 360, 270, 180, 90, 180,
        270, 360, 450, 540, 630,
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("a_jj_play_baseball/{i}.png"))?;
        let (x, y) = locs[i];
        let img = images[0].circle().resize_exact((66, 66)).rotate(angles[i] as f32);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&img, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 23, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "a_jj_play_baseball",
    a_jj_play_baseball,
    min_images = 1,
    max_images = 1,
    keywords = &["打棒球"],
    date_created = local_date(2025, 5, 15),
    date_modified = local_date(2025, 5, 15),
}
