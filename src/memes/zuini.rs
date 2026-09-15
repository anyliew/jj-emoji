use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn zuini(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let one_locs = [
        (-5, -5, 260, 260), (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240),
        (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240),
        (-5, -5, 260, 260), (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240),
        (0, 0, 240, 240), (0, 0, 240, 240), (-5, -5, 260, 260), (0, 0, 240, 240), (0, 0, 240, 240),
        (0, 0, 240, 240), (0, 0, 240, 240), (0, 0, 240, 240),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("zuini/{i}.png"))?;
        let img = images[0].square().transparency(80.0 / 255.0);
        let (x, y, w, h) = one_locs[i];
        let resized = img.resize_exact((w, h));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_image(&resized, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 23, duration: 0.04 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "zuini",
    zuini,
    min_images = 1,
    max_images = 1,
    keywords = &["嘴你"],
    date_created = local_date(2025, 6, 11),
    date_modified = local_date(2025, 6, 11),
}
