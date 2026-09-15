use skia_safe::{Color, Image, ISize};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
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

fn electrify_you(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!("电死你个 {}", texts.first().map(|s| s.as_str()).unwrap_or("猪头"));
    let locs = [(68, 82), (62, 80)];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("electrify_you/{i}.png"))?;
        let img = images[0].circle().resize_exact((118, 85));
        let t = text_image(
            &text,
            25.0,
            440.0,
            text_params!(
                font_families = &["033-SSFangTangTi"],
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 0), 1.0),
            ),
        );
        let x = (png.width() - t.width()) / 2;
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&img, locs[i], None);
        canvas.draw_image(&png, (0, 0), None);
        canvas.draw_image(&t, (x, 12), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 2, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "electrify_you",
    electrify_you,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["猪头"],
    keywords = &["电死你"],
    date_created = local_date(2025, 5, 20),
    date_modified = local_date(2025, 5, 20),
}
