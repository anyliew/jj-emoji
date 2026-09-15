use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn richu(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (78, 104), (82, 105), (84, 102), (89, 97), (91, 90),
        (94, 82), (98, 76), (102, 69), (104, 65), (106, 60),
        (107, 58), (107, 58),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("richu/{i}.png"))?;
        let (x, y) = locs[i];
        let r = images[0].circle().resize_exact((80, 80));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&r, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 12, duration: 0.1 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "richu",
    richu,
    min_images = 1,
    max_images = 1,
    keywords = &["日出"],
    date_created = local_date(2025, 6, 17),
    date_modified = local_date(2025, 6, 17),
}
