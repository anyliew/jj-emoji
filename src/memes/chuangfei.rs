use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn chuangfei(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("chuangfei/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((100, 100));
        let (paste_x, paste_y, rotated) = if i < 6 {
            (77, 54, img)
        } else if i < 20 {
            let move_dist = 5 * (i as i32 - 5);
            (77 - move_dist, 54 - move_dist, img.rotate(30.0 * (i as i32 - 5) as f32))
        } else {
            (-100, -100, img)
        };
        let new_x = paste_x - (rotated.width() - 100) / 2;
        let new_y = paste_y - (rotated.height() - 100) / 2;
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&rotated, (new_x, new_y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 35, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "chuangfei",
    chuangfei,
    min_images = 1,
    max_images = 1,
    keywords = &["创飞"],
    date_created = local_date(2025, 5, 15),
    date_modified = local_date(2025, 5, 15),
}
