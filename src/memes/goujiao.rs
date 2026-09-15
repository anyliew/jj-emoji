use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct GoujiaoOptions {
    #[option(short, long, description = "指定文本")]
    name: Option<String>,
}

fn goujiao(images: Vec<InputImage>, texts: Vec<String>, options: GoujiaoOptions) -> Result<Vec<u8>, Error> {
    let mut name = options.name.clone().unwrap_or_else(|| "不服你也爆".to_string());
    if name.chars().count() > 10 {
        name = name.chars().take(10).collect();
    }
    let text = &texts[0];

    let func = |images: Vec<Image>| {
        let frame_bg = load_image("goujiao/0.png")?;
        let img = images[0].circle().resize_exact((78, 78));
        let mut surface = new_surface(frame_bg.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame_bg, (0, 0), None);
        canvas.draw_image(&img, (300, 151), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(298, 136, 376, 150),
            text,
            10.0,
            25.0,
            text_params!(
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(403, 115, 477, 170),
            &name,
            10.0,
            18.0,
            text_params!(
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                text_align = TextAlign::Center,
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "goujiao",
    goujiao,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["下头豹"],
    keywords = &["狗叫"],
    date_created = local_date(2025, 9, 12),
    date_modified = local_date(2025, 9, 12),
}
