use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_str, load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn xiatou(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = texts.first().map(|s| s.as_str()).unwrap_or("这个群友，蒸丅亠！");
    let img = images[0].image.clone().resize_exact((30, 30));
    let mut text2image = Text2Image::from_text(
        text,
        24.0,
        text_params!(
            font_families = &["System"],
            paint = new_paint(Color::from_rgb(0, 0, 0)),
            stroke_paint = new_stroke_paint(color_from_str("#d6e5fc"), 0.2),
        ),
    );
    text2image.layout(330.0);
    if text2image.height() > 45.0 {
        return Err(Error::TextOverLength(text.to_string()));
    }

    let mut encoder = GifEncoder::new();
    for i in 0..3 {
        let png = load_image(format!("xiatou/{i}.png"))?;
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&png, (0, 0), None);
        canvas.draw_image(&img, (35, 33), None);
        let text_w = text2image.longest_line().ceil() as i32;
        let x = (png.width() - text_w) / 2;
        text2image.draw_on_canvas(&canvas, (x, 218));
        let duration = if i < 2 { 0.3 } else { 1.0 };
        encoder.add_frame(surface.image_snapshot(), duration)?;
    }
    encoder.finish()
}

register_meme! {
    "xiatou",
    xiatou,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["这个群友，蒸丅亠！"],
    keywords = &["丅亠"],
    date_created = local_date(2025, 5, 22),
    date_modified = local_date(2025, 5, 22),
}
