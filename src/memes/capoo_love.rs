use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn capoo_love(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (62, 133, 62, 62), (62, 133, 62, 62), (62, 133, 62, 62), (62, 133, 62, 62),
        (62, 133, 62, 62), (62, 133, 62, 62), (-62, -62, 62, 62), (-62, -62, 62, 62),
        (-62, -62, 62, 62), (-62, -62, 62, 62), (-62, -62, 62, 62), (85, 91, 45, 40),
        (73, 96, 45, 40), (61, 131, 65, 65),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("capoo_love/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        let img = images[0].circle().resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 14, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "capoo_love",
    capoo_love,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波爱心", "❤️"],
    date_created = local_date(2025, 6, 6),
    date_modified = local_date(2025, 6, 6),
}
