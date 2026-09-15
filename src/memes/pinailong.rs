use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pinailong(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let one_locs = [
        (91, 60, 100, 100), (158, 89, 100, 100), (154, 64, 90, 90), (172, 76, 90, 90), (186, 76, 90, 88),
        (179, 114, 90, 88), (85, 36, 85, 85), (93, 44, 85, 85), (93, 107, 85, 85), (70, 132, 85, 85),
        (90, 102, 85, 85), (99, 94, 95, 85), (93, 87, 110, 100), (61, 72, 140, 130), (40, 62, 150, 145),
        (-20, 93, 180, 160), (54, 75, 150, 150), (62, 59, 155, 140), (60, 55, 155, 140), (73, 51, 148, 140),
        (69, 55, 145, 140), (72, 50, 145, 140), (68, 52, 145, 140), (68, 52, 145, 140),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("pinailong/{i}.png"))?;
        let (x, y, w, h) = one_locs[i];
        let r = images[0].circle().resize_exact((w, h));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_image(&r, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 24, duration: 0.12 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "pinailong",
    pinailong,
    min_images = 1,
    max_images = 1,
    keywords = &["劈奶龙"],
    date_created = local_date(2025, 5, 31),
    date_modified = local_date(2025, 5, 31),
}
