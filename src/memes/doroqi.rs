use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doroqi(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (125, 121), (114, 135), (113, 130), (109, 138), (108, 134), (105, 140), (105, 137),
        (103, 141), (100, 144), (108, 129), (117, 121), (111, 126), (121, 114), (116, 118),
        (119, 113), (118, 116), (104, 125), (106, 126), (97, 137),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("doroqi/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((80, 80));
        canvas.draw_image(&img, locs[i], None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 19, duration: 0.1 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "doroqi",
    doroqi,
    min_images = 1,
    max_images = 1,
    keywords = &["doro骑"],
    date_created = local_date(2025, 8, 6),
    date_modified = local_date(2025, 8, 6),
}
