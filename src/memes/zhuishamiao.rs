use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn zhuishamiao(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (23, 59), (24, 60), (29, 57), (32, 60), (34, 58), (43, 60),
        (46, 60), (43, 58), (43, 60), (43, 63), (41, 62),
        (39, 64), (45, 61), (46, 55), (43, 57), (41, 56), (40, 52), (37, 52),
        (33, 49), (33, 53), (37, 50), (39, 49), (41, 49), (44, 50),
        (44, 52),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("zhuishamiao/{i}.png"))?;
        let img = images[0].circle().resize_exact((60, 48));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, locs[i], None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 25, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "zhuishamiao",
    zhuishamiao,
    min_images = 1,
    max_images = 1,
    keywords = &["追杀喵"],
    date_created = local_date(2025, 5, 28),
    date_modified = local_date(2025, 5, 28),
}
