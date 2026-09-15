use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn zhongcheng(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = texts.first().map(|s| s.as_str()).unwrap_or("华为");

    let func = |images: Vec<Image>| {
        let frame = load_image("zhongcheng/0.png")?;
        let mut text2image = Text2Image::from_text(
            text,
            35.0,
            text_params!(
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        );
        text2image.layout(120.0);
        if text2image.height() > 70.0 {
            return Err(Error::TextOverLength(text.to_string()));
        }
        let img = images[0].circle().resize_fit((90, 90), Fit::Contain);
        let user_head = if images.len() > 1 {
            Some(images[1].circle().resize_exact((80, 80)))
        } else {
            None
        };
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        text2image.draw_on_canvas(&canvas, (202, 20));
        canvas.draw_image(&img, (112, 74), None);
        if let Some(uh) = &user_head {
            canvas.draw_image(uh, (115, 172), None);
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "zhongcheng",
    zhongcheng,
    min_images = 1,
    max_images = 2,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["华为"],
    keywords = &["忠诚"],
    date_created = local_date(2025, 6, 23),
    date_modified = local_date(2025, 6, 23),
}
