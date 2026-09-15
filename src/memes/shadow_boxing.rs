use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn shadow_boxing(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (133, 90), (133, 90), (133, 90), (120, 90), (108, 90), (108, 90),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("shadow_boxing/{i}.png"))?;
        let (x, y) = locs[i];
        let r = images[0].circle().resize_exact((60, 60));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_image(&r, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 6, duration: 0.15 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "shadow_boxing",
    shadow_boxing,
    min_images = 1,
    max_images = 1,
    keywords = &["太极"],
    date_created = local_date(2025, 5, 14),
    date_modified = local_date(2025, 5, 14),
}
