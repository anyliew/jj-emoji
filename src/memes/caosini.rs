use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn caosini(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let head_locs = [
        (68, 169), (68, 172), (151, 191), (164, 146), (178, 131), (98, 222), (98, 224),
        (83, 145), (83, 145),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("caosini/{i}.png"))?;
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        let img = images[0].resize_exact((85, 90));
        canvas.draw_image(&img, head_locs[i], None);
        canvas.draw_image(&png, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 9, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "caosini",
    caosini,
    min_images = 1,
    max_images = 1,
    keywords = &["炒你"],
    date_created = local_date(2025, 5, 20),
    date_modified = local_date(2025, 5, 20),
}
