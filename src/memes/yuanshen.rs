use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn yuanshen(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (-60, 0, 1, 1), (106, 137, 18, 13), (114, 100, 26, 31), (80, -5, 75, 89), (79, -10, 75, 80), (85, 2, 62, 55),
        (87, 2, 68, 68), (87, 1, 68, 64), (85, -4, 70, 72), (89, 0, 65, 66), (91, 2, 58, 57), (88, 0, 64, 62),
        (90, 1, 62, 60), (90, 2, 58, 57), (90, 2, 58, 57), (96, 6, 56, 58), (106, 14, 69, 65), (106, 35, 62, 58),
        (106, 48, 64, 66), (101, 54, 62, 64), (95, 52, 65, 66), (89, 46, 61, 62), (87, 48, 61, 62), (87, 48, 61, 62),
        (87, 48, 61, 62), (87, 48, 61, 62), (87, 48, 61, 62), (86, 49, 61, 62), (87, 51, 61, 62), (87, 53, 61, 62),
        (87, 50, 61, 62), (81, 44, 61, 62), (79, 39, 61, 62), (78, 42, 61, 62), (80, 45, 61, 62), (81, 46, 61, 62),
    ];
    let rotate_num = [
        0, 0, -5, -15, 25, 22, -15, -6, 12, -7,
        0, 4, -3, 0, 0, 0, -1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("yuanshen/{i}.png"))?;
        let user_head = images[0].resize_exact((60, 60));
        let rotated = user_head.rotate(-rotate_num[i] as f32);
        let (x, y, w, h) = user_locs[i];
        let resized = rotated.resize_exact((w, h));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&resized, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 36, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "yuanshen",
    yuanshen,
    min_images = 1,
    max_images = 1,
    keywords = &["缘神"],
    date_created = local_date(2025, 8, 29),
    date_modified = local_date(2025, 8, 29),
}
