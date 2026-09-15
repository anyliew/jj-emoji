use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn wuyingtui(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (78, 70), (78, 70), (75, 72), (75, 72), (78, 70), (78, 70),
        (75, 72), (75, 72), (78, 70), (78, 70), (75, 72), (75, 72),
        (78, 70), (78, 70), (78, 70), (78, 70), (75, 72), (75, 72),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("wuyingtui/{i}.png"))?;
        let img = images[0].resize_exact((34, 34));
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, locs[i], None);
        canvas.draw_image(&png, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 18, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "wuyingtui",
    wuyingtui,
    min_images = 1,
    max_images = 1,
    keywords = &["无影腿"],
    date_created = local_date(2025, 5, 21),
    date_modified = local_date(2025, 5, 21),
}
