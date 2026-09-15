use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn spike_spinebuster(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let one_locs = [
        (203, 22), (180, 18), (219, 25), (219, 25), (247, 31), (246, 31), (259, 36),
        (265, 36), (265, 37), (164, 41),
    ];
    let tow_locs = [
        (184, 11), (274, 13), (305, -10), (305, -10), (305, -10), (149, 21), (176, 161),
        (147, 189), (139, 173), (138, 172), (147, 137), (142, 99), (139, 95), (139, 95),
        (138, 87), (136, 88), (135, 76), (133, 76), (134, 84), (132, 92), (133, 114),
        (135, 157), (136, 161), (136, 161), (136, 161), (136, 161), (136, 161), (136, 161),
        (136, 161), (136, 161), (136, 161), (136, 161), (136, 161), (136, 161),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("spike_spinebuster/{i}.png"))?;
        let img = images[0].circle().resize_exact((50, 45));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        if i < 10 {
            let (x, y) = one_locs[i];
            canvas.draw_image(&img, (x, y), None);
        }
        if (27..=61).contains(&i) {
            let (x, y) = tow_locs[i - 27];
            canvas.draw_image(&img, (x, y), None);
        }
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 61, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "spike_spinebuster",
    spike_spinebuster,
    min_images = 1,
    max_images = 1,
    keywords = &["斯派克抱摔"],
    date_created = local_date(2025, 5, 27),
    date_modified = local_date(2025, 5, 27),
}
