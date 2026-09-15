use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn spinner(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("spinner/{i}.png"))?;
        let rotated = images[0].rotate(90.0 * i as f32).circle().resize_exact((60, 60));
        let y = if i == 13 || i == 14 { 100 } else { 120 };
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&rotated, (135, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 24, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "spinner",
    spinner,
    min_images = 1,
    max_images = 1,
    keywords = &["陀螺"],
    date_created = local_date(2025, 5, 13),
    date_modified = local_date(2025, 5, 19),
}
