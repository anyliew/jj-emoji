use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn keliplay(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let head_locs = [
        (71, 38), (71, 38), (95, 38), (113, 45), (90, 41), (66, 41), (91, 41), (111, 46),
        (87, 74), (74, 89), (61, 101), (63, 76), (66, 57), (68, 52), (67, 43), (67, 46),
        (79, 55), (66, 44), (79, 52), (90, 52), (116, 57), (131, 57), (71, 59), (78, 62),
        (93, 85), (93, 85), (99, 95), (106, 107), (130, 31), (130, 31), (130, 31), (130, 31),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("keliplay/{i}.png"))?;
        let img = images[0].resize_exact((109, 109));
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, head_locs[i], None);
        canvas.draw_image(&png, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 32, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "keliplay",
    keliplay,
    min_images = 1,
    max_images = 1,
    keywords = &["可莉打"],
    date_created = local_date(2025, 5, 19),
    date_modified = local_date(2025, 5, 19),
}
