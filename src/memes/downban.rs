use skia_safe::{Color, Image, ISize};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    text::{Text2Image, TextParams},
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct DownbanOptions {
    #[option(short, long, description = "指定时间")]
    time: Option<String>,
}

fn bbcode_image(text: &str, font_size: f32, width: f32, params: TextParams) -> Image {
    let mut t2i = Text2Image::from_bbcode_text(text, font_size, params);
    t2i.layout(width);
    let w = t2i.longest_line().ceil() as i32;
    let h = t2i.height().ceil() as i32;
    let mut surface = new_surface(ISize::new(w.max(1), h.max(1)));
    t2i.draw_on_canvas(surface.canvas(), (0, 0));
    surface.image_snapshot()
}

fn downban(images: Vec<InputImage>, _: Vec<String>, options: DownbanOptions) -> Result<Vec<u8>, Error> {
    let time = options.time.clone().unwrap_or_default();

    let func = |images: Vec<Image>| {
        let frame = load_image("downban/0.png")?;
        let img = images[0].circle().resize_exact((80, 80));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        if !time.is_empty() {
            let t = bbcode_image(
                &format!("[u]{}[/u]", time),
                30.0,
                300.0,
                text_params!(
                    paint = new_paint(Color::from_rgb(255, 255, 255)),
                ),
            );
            let r = t.rotate(-22.0);
            let x = 237 - r.width() / 2;
            let y = 298 - r.height() / 2;
            canvas.draw_image(&r, (x, y), None);
        }
        canvas.draw_image(&img, (208, 76), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "downban",
    downban,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["下班"],
    date_created = local_date(2025, 6, 13),
    date_modified = local_date(2025, 6, 14),
}
