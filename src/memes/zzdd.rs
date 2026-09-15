use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn zzdd(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (47, 63), (47, 63), (47, 63), (47, 63), (51, 126), (51, 126),
        (51, 126), (51, 126), (51, 126), (39, 105), (39, 105), (39, 105),
        (39, 105), (39, 105), (47, 63),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("zzdd/{i}.png"))?;
        let img = images[0].resize_exact((80, 80));
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, locs[i], None);
        canvas.draw_image(&png, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 15, duration: 0.04 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "zzdd",
    zzdd,
    min_images = 1,
    max_images = 1,
    keywords = &["指指点点"],
    date_created = local_date(2025, 7, 3),
    date_modified = local_date(2025, 7, 3),
}
