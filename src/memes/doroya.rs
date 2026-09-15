use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn doroya(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(88, 82), (88, 85), (52, 89), (90, 77), (84, 88), (52, 88)];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("doroya/{i}.png"))?;
        let img = images[0].circle().resize_exact((192, 156));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&img, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 6, duration: 0.1 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "doroya",
    doroya,
    min_images = 1,
    max_images = 1,
    keywords = &["doro鸭"],
    date_created = local_date(2025, 5, 19),
    date_modified = local_date(2025, 5, 19),
}
