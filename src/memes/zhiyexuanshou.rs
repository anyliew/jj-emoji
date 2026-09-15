use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct ZhiyexuanshouOptions {
    #[option(short, long, description = "指定名字")]
    name: Option<String>,
}

fn zhiyexuanshou(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: ZhiyexuanshouOptions,
) -> Result<Vec<u8>, Error> {
    let name = options
        .name
        .clone()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "某某职业选手".to_string());

    let (x0, y0, x1, y1) = (162, 14, 214, 275);
    let box_w = x1 - x0;
    let box_h = y1 - y0;

    let mut font_size = 20.0;
    let mut char_widths: Vec<f32>;
    let mut char_heights: Vec<f32>;
    loop {
        let mut t2is: Vec<Text2Image> = name
            .chars()
            .map(|c| {
                let mut t2i = Text2Image::from_text(
                    &c.to_string(),
                    font_size,
                    text_params!(paint = new_paint(Color::from_rgb(0, 0, 0))),
                );
                t2i.layout(10000.0);
                t2i
            })
            .collect();
        char_widths = t2is.iter_mut().map(|t| t.longest_line()).collect();
        char_heights = t2is.iter().map(|t| t.height()).collect();
        let max_w = char_widths.iter().cloned().fold(0.0, f32::max);
        let total_h: f32 = char_heights.iter().sum();
        if (max_w <= box_w as f32 && total_h <= box_h as f32) || font_size <= 10.0 {
            break;
        }
        font_size -= 1.0;
    }
    let max_w = char_widths.iter().cloned().fold(0.0, f32::max);
    let total_h: f32 = char_heights.iter().sum();

    let func = |images: Vec<Image>| {
        let background = load_image("zhiyexuanshou/0.png")?;
        let user_img = images[0].circle().resize_exact((50, 50));
        let mut surface = new_surface(background.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&background, (0, 0), None);
        canvas.draw_image(&user_img, (64, 3), None);

        let mut y = y0 as f32 + ((box_h as f32 - total_h) / 2.0).ceil();
        for (i, ch) in name.chars().enumerate() {
            let mut t2i = Text2Image::from_text(
                &ch.to_string(),
                font_size,
                text_params!(paint = new_paint(Color::from_rgb(0, 0, 0))),
            );
            t2i.layout(10000.0);
            let w = char_widths[i];
            let x = x0 + ((box_w as f32 - max_w) / 2.0).ceil() as i32
                + ((max_w - w) / 2.0).ceil() as i32;
            t2i.draw_on_canvas(canvas, (x, y as i32));
            y += char_heights[i];
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
