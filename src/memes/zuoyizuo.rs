use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn zuoyizuo(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (78, 140, 95, 100), (76, 130, 100, 110), (76, 140, 100, 110), (78, 140, 95, 100), (76, 130, 100, 110),
        (76, 130, 100, 110), (78, 140, 95, 100), (78, 140, 95, 100), (76, 130, 100, 110), (75, 130, 100, 110),
        (76, 130, 100, 110), (75, 130, 100, 110), (76, 130, 100, 110), (76, 130, 100, 110), (75, 130, 100, 110),
        (78, 130, 95, 100), (78, 140, 95, 100), (78, 140, 95, 100),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("zuoyizuo/{i}.png"))?;
        let (x, y, w, h) = locs[i];
        let img = images[0].square().resize_exact((w, h));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 18, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "zuoyizuo",
    zuoyizuo,
    min_images = 1,
    max_images = 1,
    keywords = &["唑一唑"],
    date_created = local_date(2025, 6, 17),
    date_modified = local_date(2025, 6, 17),
}
