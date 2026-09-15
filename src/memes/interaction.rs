use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn interaction(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (156, 90), (156, 86), (156, 90), (156, 86), (156, 93), (156, 103), (156, 93), (156, 103),
        (156, 93), (69, 103), (31, 103), (25, 103), (25, 103),
    ];
    let self_locs = [
        (17, 90), (17, 83), (17, 90), (17, 83), (17, 102), (17, 93), (17, 102), (17, 93),
        (17, 102), (115, 93), (140, 93), (146, 93), (146, 93),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("interaction/{i}.png"))?;
        let user = images[1].circle().resize_exact((100, 90));
        let self_head = images[0].circle().resize_exact((100, 90));
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&png, (0, 0), None);
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
    "interaction",
    interaction,
    min_images = 2,
    max_images = 2,
    keywords = &["互动"],
    date_created = local_date(2025, 5, 12),
    date_modified = local_date(2025, 5, 12),
}
