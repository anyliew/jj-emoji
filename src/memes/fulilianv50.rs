use skia_safe::{Color, IRect, Image};

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

fn fulilianv50(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = texts.first().map(|s| s.as_str()).unwrap_or("我是芙蓉王，请v我50");

    let func = |images: Vec<Image>| {
        let frame = load_image("fulilianv50/v50.png")?;
        let img = images[0].circle().resize_exact((120, 120));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (120, 89), None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(9, 14, frame.width() - 9, 60),
            text,
            18.0,
            25.0,
            text_params!(
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "fulilianv50",
    fulilianv50,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["我是芙蓉王，请v我50"],
    keywords = &["芙莉莲v50"],
    date_created = local_date(2025, 5, 22),
    date_modified = local_date(2025, 5, 22),
}
