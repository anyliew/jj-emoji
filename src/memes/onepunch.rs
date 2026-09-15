use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn onepunch(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("onepunch/{i}.png"))?;
        let img = images[0].resize_exact((218, 211));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        if (4..=7).contains(&i) {
            let r = img.resize_exact((208, 201));
            canvas.draw_image(&r, (10, 10), None);
        } else {
            canvas.draw_image(&img, (0, 0), None);
        }
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 8, duration: 0.02 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "onepunch",
    onepunch,
    min_images = 1,
    max_images = 1,
    keywords = &["给你一拳"],
    date_created = local_date(2025, 5, 16),
    date_modified = local_date(2025, 5, 16),
}
