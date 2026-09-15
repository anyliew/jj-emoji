use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn yesirmiao(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (111, 48, 125, 110), (99, 42, 125, 110), (91, 43, 125, 110), (86, 43, 125, 110),
        (79, 39, 125, 110), (71, 41, 125, 110), (62, 43, 125, 110), (56, 44, 125, 110),
        (47, 45, 125, 110), (37, 44, 125, 110),
    ];
    let tow_locs = [
        (25, 42, 125, 100), (13, 42, 125, 100), (6, 41, 125, 100), (4, 43, 125, 100),
        (5, 50, 125, 100), (6, 56, 125, 110), (11, 65, 125, 100), (16, 71, 125, 100),
        (24, 78, 125, 100), (33, 66, 125, 100), (37, 31, 125, 100), (44, 15, 125, 90),
        (47, 21, 125, 90), (54, 26, 125, 90), (56, 32, 125, 90),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("yesirmiao/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        if i < locs.len() {
            let (x, y, w, h) = locs[i];
            let img = images[0].circle().resize_exact((w, h));
            canvas.draw_image(&img, (x, y), None);
        }
        if (10..=25).contains(&i) && i - 10 < tow_locs.len() {
            let (x, y, w, h) = tow_locs[i - 10];
            let img = images[0].circle().resize_exact((w, h));
            canvas.draw_image(&img, (x, y), None);
        }
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
    "yesirmiao",
    yesirmiao,
    min_images = 1,
    max_images = 1,
    keywords = &["敬礼喵"],
    date_created = local_date(2025, 5, 28),
    date_modified = local_date(2025, 5, 28),
}
