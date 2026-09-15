use skia_safe::{textlayout::TextAlign, Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn zhiyexuanshou(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = "某某职业选手";

    let func = |images: Vec<Image>| {
        let background = load_image("zhiyexuanshou/0.png")?;
        let user_img = images[0].circle().resize_exact((50, 50));
        let mut surface = new_surface(background.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&background, (0, 0), None);
        canvas.draw_image(&user_img, (64, 3), None);

        let chars: Vec<String> = name.chars().map(|c| c.to_string()).collect();
        let count = chars.len().max(1);
        let char_h = (275 - 14) / count as i32;
        for (i, ch) in chars.iter().enumerate() {
            let top = 14 + i as i32 * char_h;
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(162, top, 214, top + char_h),
                ch,
                10.0,
                20.0,
                text_params!(
                    font_families = &["GlowSansSC-Normal-Heavy"],
                    text_align = TextAlign::Center,
                    paint = new_paint(Color::from_rgb(0, 0, 0)),
                ),
            )?;
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "zhiyexuanshou",
    zhiyexuanshou,
    min_images = 1,
    max_images = 1,
    keywords = &["职业选手"],
    date_created = local_date(2025, 9, 11),
    date_modified = local_date(2025, 9, 11),
}
