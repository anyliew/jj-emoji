use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn get_up(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (7, 62), (9, 60), (9, 60), (9, 60), (9, 60), (9, 60), (7, 62), (7, 62), (7, 62), (7, 62),
        (7, 62), (7, 62), (7, 62), (7, 62), (7, 62), (7, 62), (7, 62), (7, 62), (7, 62), (7, 62),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("get_up/{i}.png"))?;
        let img = images[0].circle().resize_exact((100, 100));
        let rotated = img.rotate(-45.0);
        let new_x = locs[i].0 - (rotated.width() - 100) / 2;
        let new_y = locs[i].1 - (rotated.height() - 100) / 2;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&rotated, (new_x, new_y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 20, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "get_up",
    get_up,
    min_images = 1,
    max_images = 1,
    keywords = &["起床"],
    date_created = local_date(2025, 5, 14),
    date_modified = local_date(2025, 5, 14),
}
