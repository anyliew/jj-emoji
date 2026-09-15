use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn chiikawa(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (92, 142), (90, 185), (86, 194), (86, 194), (87, 189), (87, 144), (92, 134), (90, 141),
        (90, 192), (96, 197), (85, 186), (90, 170), (93, 132),
    ];
    let self_locs = [
        (219, 142), (217, 185), (213, 194), (213, 194), (214, 184), (214, 144), (219, 134),
        (217, 141), (217, 192), (214, 197), (212, 186), (226, 170), (220, 132),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("chiikawa/{i}.png"))?;
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let user = images[1].circle().resize_exact((100, 90));
        let self_head = images[0].circle().resize_exact((100, 90));
        canvas.draw_image(&user, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 13, duration: 0.1 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "chiikawa",
    chiikawa,
    min_images = 2,
    max_images = 2,
    keywords = &["吉伊卡哇"],
    date_created = local_date(2025, 5, 22),
    date_modified = local_date(2025, 5, 22),
}
