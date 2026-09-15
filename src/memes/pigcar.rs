use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pigcar(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [(87, 62), (86, 71)];
    let user_locs = [(75, 189), (76, 195)];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("pigcar/{i}.png"))?;
        let self_head = images[0].circle().resize_exact((140, 110));
        let user_head = images[1].circle().resize_exact((85, 80));
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let (ux, uy) = user_locs[i];
        let (sx, sy) = self_locs[i];
        canvas.draw_image(&user_head, (ux, uy), None);
        canvas.draw_image(&self_head, (sx, sy), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 2, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "pigcar",
    pigcar,
    min_images = 2,
    max_images = 2,
    keywords = &["猪猪车"],
    date_created = local_date(2025, 5, 23),
    date_modified = local_date(2025, 5, 23),
}
