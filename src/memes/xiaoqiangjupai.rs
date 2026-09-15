use skia_safe::{Color, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn xiaoqiangjupai(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = texts.first().map(|s| s.as_str()).unwrap_or("活力大湾区，魅力新广州！");
    let mut size = 50;
    let mut chosen: Option<Text2Image> = None;
    while size >= 10 {
        let mut t = Text2Image::from_text(
            text,
            size as f32,
            text_params!(
                paint = new_paint(Color::from_rgb(105, 61, 36)),
                text_align = TextAlign::Left,
            ),
        );
        t.layout(181.0);
        if t.height() <= 150.0 && t.longest_line() <= 181.0 {
            chosen = Some(t);
            break;
        }
        size -= 1;
    }
    let text2image = chosen.ok_or_else(|| Error::TextOverLength(text.to_string()))?;

    let frame_coords = [
        (131, 25), (164, 19), (181, 31), (185, 42), (184, 57), (185, 66),
        (187, 68), (187, 65), (185, 61), (184, 58), (185, 57),
    ];

    let mut encoder = GifEncoder::new();
    for i in 0..18 {
        let png = load_image(format!("xiaoqiangjupai/{i}.png"))?;
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&png, (0, 0), None);
        if i >= 7 {
            let (x, y) = frame_coords[i - 7];
            text2image.draw_on_canvas(&canvas, (x, y));
        }
        let duration = if i < 17 { 0.08 } else { 1.0 };
        encoder.add_frame(surface.image_snapshot(), duration)?;
    }
    encoder.finish()
}

register_meme! {
    "xiaoqiangjupai",
    xiaoqiangjupai,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["活力大湾区，魅力新广州！"],
    keywords = &["小强举牌"],
    date_created = local_date(2025, 12, 3),
    date_modified = local_date(2025, 12, 4),
}
