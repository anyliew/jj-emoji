use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dorochui(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [(73, 72, 120, 90), (117, 48, 120, 90)];
    let self_locs = [(19, 197, 100, 65), (1, 157, 120, 90)];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("dorochui/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let (x1, y1, w1, h1) = user_locs[i];
        let (x2, y2, w2, h2) = self_locs[i];
        let self_head = images[0].circle().resize_exact((w1, h1));
        let user_head = images[1].circle().resize_exact((w2, h2));
        canvas.draw_image(&self_head, (x1, y1), None);
        canvas.draw_image(&user_head, (x2, y2), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 2, duration: 0.02 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "dorochui",
    dorochui,
    min_images = 2,
    max_images = 2,
    keywords = &["doro锤"],
    date_created = local_date(2025, 6, 9),
    date_modified = local_date(2025, 6, 9),
}
