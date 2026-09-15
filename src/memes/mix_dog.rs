use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn mix_dog(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (21, 90), (32, 50), (32, 16), (44, 9), (31, 16), (21, 27), (21, 88), (0, 80),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("mix_dog/{i}.png"))?;
        let img = images[0].circle().resize_exact((85, 85));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&img, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 8, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "mix_dog",
    mix_dog,
    min_images = 1,
    max_images = 1,
    keywords = &["小狗"],
    date_created = local_date(2025, 5, 14),
    date_modified = local_date(2025, 5, 14),
}
