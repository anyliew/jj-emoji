use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn slipper(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image("slipper/ditu.png")?;
        let overlay = load_image(format!("slipper/{i}.png"))?;
        let r = images[0].circle().resize_exact((74, 74));
        let (px, py, rot) = if i >= 2 {
            (
                20 - 8 * (i - 2) as i32,
                120 + 10 * (i - 2) as i32,
                100.0 * (i - 2) as f32,
            )
        } else {
            (20, 120, 0.0)
        };
        let rotated = if i >= 2 { r.rotate_crop(rot) } else { r };
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&rotated, (px, py), None);
        canvas.draw_image(&overlay, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 16, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "slipper",
    slipper,
    min_images = 1,
    max_images = 1,
    keywords = &["拖鞋"],
    date_created = local_date(2025, 5, 13),
    date_modified = local_date(2025, 5, 13),
}
