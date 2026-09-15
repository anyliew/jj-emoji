use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dorotuodi(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(81, 64), (81, 60), (78, 64), (78, 59), (78, 58)];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("dorotuodi/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        let img = images[0].circle().resize_exact((61, 55));
        canvas.draw_image(&img, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 5, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "dorotuodi",
    dorotuodi,
    min_images = 1,
    max_images = 1,
    keywords = &["doro拖地"],
    date_created = local_date(2025, 5, 19),
    date_modified = local_date(2025, 5, 19),
}
