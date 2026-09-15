use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct SiketeOptions {
    #[option(short, long, description = "指定名字")]
    name: Option<String>,
}

fn sikete(images: Vec<InputImage>, texts: Vec<String>, options: SiketeOptions) -> Result<Vec<u8>, Error> {
    let name = options
        .name
        .clone()
        .map(|n| n.chars().take(20).collect::<String>())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "正义斯科特".to_string());
    let text = if texts.is_empty() { "那我的屁股怎么办" } else { &texts[0] };
    let img = images[0].image.circle().resize_exact((260, 230));
    let frame_bg = load_image("sikete/0.png")?;
    let mut surface = new_surface(frame_bg.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(&img, (135, 31), None);
    canvas.draw_image(&frame_bg, (0, 0), None);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(136, 382, 360, 437),
        text,
        18.0,
        18.0,
        text_params!(
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(255, 255, 255)),
        ),
    )?;
    let mut t2i = Text2Image::from_text(
        &name,
        18.0,
        text_params!(paint = new_paint(Color::from_rgb(232, 214, 173))),
    );
    t2i.layout(10000.0);
    let name_w = t2i.longest_line().ceil() as i32;
    t2i.draw_on_canvas(canvas, ((frame_bg.width() - name_w) / 2 - 80, 348));
    encode_png(surface.image_snapshot())
}

register_meme! {
    "sikete",
    sikete,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["那我的屁股怎么办"],
    keywords = &["斯科特"],
    date_created = local_date(2025, 7, 13),
    date_modified = local_date(2025, 7, 13),
}
