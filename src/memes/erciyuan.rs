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
pub(crate) struct ErciyuanOptions {
    #[option(short, long, description = "指定名字")]
    name: Option<String>,
}

fn erciyuan(images: Vec<InputImage>, _: Vec<String>, options: ErciyuanOptions) -> Result<Vec<u8>, Error> {
    let name = options.name.clone().unwrap_or_else(|| "二次元".to_string());

    let func = |images: Vec<Image>| {
        let background = load_image("erciyuan/0.png")?;
        let user = images[0].circle().resize_exact((40, 40));
        let mut surface = new_surface(background.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&background, (0, 0), None);
        canvas.draw_image(&user, (12, 117), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(3, 92, 63, 115),
            &name,
            10.0,
            20.0,
            text_params!(
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        )?;
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "erciyuan",
    erciyuan,
    min_images = 1,
    max_images = 1,
    keywords = &["二次元"],
    date_created = local_date(2025, 9, 5),
    date_modified = local_date(2025, 9, 5),
}
