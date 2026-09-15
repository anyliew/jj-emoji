use skia_safe::{Color, Image, ISize};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    text::{Text2Image, TextParams},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn text_image(text: &str, font_size: f32, wrap: f32, params: TextParams) -> Image {
    let mut t2i = Text2Image::from_text(text, font_size, params);
    t2i.layout(wrap);
    let w = t2i.longest_line().ceil() as i32;
    let h = t2i.height().ceil() as i32;
    let mut surface = new_surface(ISize::new(w.max(1), h.max(1)));
    t2i.draw_on_canvas(surface.canvas(), (0, 0));
    surface.image_snapshot()
}

fn my_certificate(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!("这是我的{}证", texts.first().map(|s| s.as_str()).unwrap_or("牛马"));

    let func = |images: Vec<Image>| {
        let frame = load_image("my_certificate/niuma.png")?;
        let circle = images[0].resize_exact((150, 150)).circle();
        let square = images[0].resize_exact((31, 33));
        let t = text_image(
            &text,
            40.0,
            440.0,
            text_params!(
                paint = new_paint(Color::from_rgb(255, 255, 255)),
                stroke_paint = new_stroke_paint(Color::from_rgb(0, 0, 0), 1.0),
            ),
        );
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&circle, (94, 60), None);
        canvas.draw_image(&square, (258, 207), None);
        canvas.draw_image(&t, (20, 268), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "my_certificate",
    my_certificate,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["牛马"],
    keywords = &["我的证"],
    date_created = local_date(2025, 5, 14),
    date_modified = local_date(2025, 5, 14),
}
