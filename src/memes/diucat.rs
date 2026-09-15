use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn diucat(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let one_locs = [
        (95, 250, 80, 80), (101, 240, 80, 80), (102, 226, 80, 80), (99, 208, 80, 80),
        (94, 195, 80, 80), (80, 187, 80, 80), (82, 188, 60, 60), (65, 172, 60, 60), (53, 151, 60, 60),
    ];
    let tow_locs = [(80, 34, 30, 30), (65, 45, 30, 30), (65, 45, 30, 30)];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("diucat/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&bg, (0, 0), None);
        if i < 9 {
            let (x1, y1, w1, h1) = one_locs[i];
            let rotated = images[0].circle().resize_exact((w1, h1)).rotate(160.0);
            let new_x = x1 - (rotated.width() - w1) / 2;
            let new_y = y1 - (rotated.height() - h1) / 2;
            canvas.draw_image(&rotated, (new_x, new_y), None);
        }
        if (16..=18).contains(&i) {
            let (x2, y2, w2, h2) = tow_locs[i - 16];
            let rotated = images[0].circle().resize_exact((w2, h2)).rotate(-15.0);
            let new_x = x2 - (rotated.width() - w2) / 2;
            let new_y = y2 - (rotated.height() - h2) / 2;
            canvas.draw_image(&rotated, (new_x, new_y), None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 27, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "diucat",
    diucat,
    min_images = 1,
    max_images = 1,
    keywords = &["丢猫"],
    date_created = local_date(2025, 5, 30),
    date_modified = local_date(2025, 5, 30),
}
