use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn qunchao(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let background = load_image("qunchao/0.png")?;

    let func = |imgs: Vec<Image>| {
        let img = imgs[0].resize_exact((183, 184));
        let mut surface = new_surface(background.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (197, 198), None);
        canvas.draw_image(&background, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "qunchao",
    qunchao,
    min_images = 1,
    max_images = 1,
    keywords = &["群嘲", "笑他"],
    date_created = local_date(2025, 7, 15),
    date_modified = local_date(2025, 7, 15),
}
