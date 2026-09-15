use skia_safe::{Color, Image, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{color_from_str, load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn vni50(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = texts.first().map(|s| s.as_str()).unwrap_or("给你V50");

    let func = |images: Vec<Image>| {
        let base = load_image("vni50/0.png")?;
        let mut fsurf = base.to_surface();
        let fcanvas = fsurf.canvas();
        fcanvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(29, 184, 222, 239),
            text,
            18.0,
            25.0,
            text_params!(
                paint = new_paint(color_from_str("#fce4b8")),
            ),
        )?;
        let frame = fsurf.image_snapshot();
        let img = images[0].circle().resize_fit((60, 60), Fit::Cover);
        let mut surface = new_surface(base.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, (97, 62), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "vni50",
    vni50,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["给你V50"],
    keywords = &["v你50"],
    date_created = local_date(2025, 8, 7),
    date_modified = local_date(2025, 8, 7),
}
