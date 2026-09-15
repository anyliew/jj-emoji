use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn xile(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = "救我，我要洗了";
    let head = images[0].image.clone().circle().resize_exact((200, 200));
    let frame = load_image("xile/xiyiji.png")?;
    let mut text2image = Text2Image::from_text(
        text,
        35.0,
        text_params!(
            font_families = &["System"],
            paint = new_paint(Color::from_rgb(0, 0, 0)),
        ),
    );
    text2image.layout(440.0);
    if text2image.height() > 500.0 {
        return Err(Error::TextOverLength(text.to_string()));
    }

    let mut encoder = GifEncoder::new();
    for i in 0..22 {
        let angle = 60.0 * (i as f32 + 1.0);
        let rotated = head.rotate(angle);
        let paste_x = 72 + 100 - rotated.width() / 2;
        let paste_y = 97 + 100 - rotated.height() / 2;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&rotated, (paste_x, paste_y), None);
        let text_w = text2image.longest_line().ceil() as i32;
        let x = (frame.width() - text_w) / 2;
        text2image.draw_on_canvas(&canvas, (x, 16));
        let duration = if i < 3 { 0.08 } else if i < 15 { 0.05 } else { 0.15 };
        encoder.add_frame(surface.image_snapshot(), duration)?;
    }
    encoder.finish()
}

register_meme! {
    "xile",
    xile,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["救我，我要洗了"],
    keywords = &["洗了"],
    date_created = local_date(2025, 5, 21),
    date_modified = local_date(2025, 5, 21),
}
