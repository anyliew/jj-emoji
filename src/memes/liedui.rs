use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn liedui(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("liedui/0.png")?;

    let func = |images: Vec<Image>| {
        let user = &images[0];
        let bg_width = frame.width();
        let bg_height = frame.height();
        let mut x = 0;
        let mut size = 1;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        loop {
            if x >= bg_width {
                break;
            }
            let cur = user.resize_fit((size, size), Fit::Cover);
            canvas.draw_image(&cur, (x, 0), None);
            x += cur.width() - 10;
            size += 3;
            if x + size > bg_height + 500 {
                break;
            }
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "liedui",
    liedui,
    min_images = 1,
    max_images = 1,
    keywords = &["列队"],
    date_created = local_date(2025, 6, 19),
    date_modified = local_date(2025, 6, 19),
}
