use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::tags::MemeTags;
use crate::{options::NoOptions, register_meme};

fn qunyoujupai(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = "晓楠嬢";
    let text = if texts.is_empty() { "我是晓楠嬢" } else { &texts[0] };
    let img = images[0].image.resize_exact((425, 425));
    let frame = load_image("qunyoujupai/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.draw_image(&img, (300, 187), None);
    canvas.draw_image(&frame, (0, 0), None);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(218, 728, 768, 1104),
        text,
        18.0,
        72.0,
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            text_align = TextAlign::Center,
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    )?;
    let mut t2i = Text2Image::from_text(
        name,
        72.0,
        text_params!(paint = new_paint(Color::from_rgb(27, 27, 27))),
    );
    t2i.layout(10000.0);
    let w = t2i.longest_line().ceil() as i32;
    t2i.draw_on_canvas(canvas, ((frame.width() - w) / 2, 26));
    encode_png(surface.image_snapshot())
}

register_meme! {
    "qunyoujupai",
    qunyoujupai,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我是晓楠嬢"],
    keywords = &["群友举牌", "他举牌", "你举牌"],
    tags = MemeTags::mihoyo(),
    date_created = local_date(2025, 6, 10),
    date_modified = local_date(2025, 6, 19),
}
