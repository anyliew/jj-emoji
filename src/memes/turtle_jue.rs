use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn turtle_jue(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [(3, 52), (2, 54), (-2, 54), (2, 54)];
    let self_locs = [(33, 16), (20, 18), (18, 14), (28, 16)];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("turtle_jue/{i}.png"))?;
        let user_head = images[1].circle().resize_exact((29, 29));
        let self_head = images[0].circle().resize_exact((29, 29));
        let user_rotated = user_head.rotate(-120.0);
        let user_x = user_locs[i].0 - (user_rotated.width() - 29) / 2;
        let user_y = user_locs[i].1 - (user_rotated.height() - 29) / 2;
        let self_rotated = self_head.rotate(-30.0);
        let self_x = self_locs[i].0 - (self_rotated.width() - 29) / 2;
        let self_y = self_locs[i].1 - (self_rotated.height() - 29) / 2;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_rotated, (user_x, user_y), None);
        canvas.draw_image(&self_rotated, (self_x, self_y), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 4, duration: 0.02 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "turtle_jue",
    turtle_jue,
    min_images = 2,
    max_images = 2,
    keywords = &["龟龟撅"],
    date_created = local_date(2025, 5, 12),
    date_modified = local_date(2025, 5, 12),
}
