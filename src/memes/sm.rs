use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn sm(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let overlay = load_image(format!("sm/{i}.png"))?;
        let img = images[0].circle().resize_exact((74, 74));
        let mut surface = new_surface((200, 200));
        let canvas = surface.canvas();
        if i < 9 {
            canvas.draw_image(&img, (125, 88), None);
        } else {
            let rotated = img.rotate(30.0);
            canvas.draw_image(&rotated, (125, 88), None);
        }
        canvas.draw_image(&overlay, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 20, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "sm",
    sm,
    min_images = 1,
    max_images = 1,
    keywords = &["sm"],
    date_created = local_date(2025, 5, 13),
    date_modified = local_date(2025, 5, 13),
}
