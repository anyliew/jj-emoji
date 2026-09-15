use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pineapple(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let base = load_image("pineapple/0.png")?;

    let func = |imgs: Vec<Image>| {
        let img = imgs[0].resize_exact((130, 130)).circle();
        let mut surface = new_surface(base.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (92, 135), None);
        canvas.draw_image(&base, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "pineapple",
    pineapple,
    min_images = 1,
    max_images = 1,
    keywords = &["菠萝", "pineapple"],
    date_created = local_date(2024, 11, 10),
    date_modified = local_date(2024, 11, 10),
}
