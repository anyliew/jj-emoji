use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dorochou(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (330, 44), (328, 45), (319, 45), (317, 50), (322, 48), (322, 48),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("dorochou/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((140, 140));
        canvas.draw_image(&img, locs[i], None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 6, duration: 0.1 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "dorochou",
    dorochou,
    min_images = 1,
    max_images = 1,
    keywords = &["doro抽"],
    date_created = local_date(2025, 6, 2),
    date_modified = local_date(2025, 6, 2),
}
