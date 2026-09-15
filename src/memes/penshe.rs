use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn penshe(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let tow_locs = [
        (48, 45, 102, 81), (30, 34, 80, 81), (19, 17, 74, 75), (18, 11, 59, 60), (-19, -21, 52, 52),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("penshe/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        if i < 10 {
            let r = images[0].circle().resize_exact((188, 155)).rotate_crop(-43.0);
            canvas.draw_image(&r, (25, 25), None);
        } else if (11..=14).contains(&i) {
            let (x, y, w, h) = tow_locs[i - 10];
            let r = images[0].circle().resize_exact((w, h)).rotate_crop(-43.0);
            canvas.draw_image(&r, (x, y), None);
        }
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 15, duration: 0.1 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "penshe",
    penshe,
    min_images = 1,
    max_images = 1,
    keywords = &["喷射"],
    date_created = local_date(2025, 5, 31),
    date_modified = local_date(2025, 5, 31),
}
