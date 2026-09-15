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

fn dorowaimai(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!("{}专送", texts.first().map(|s| s.as_str()).unwrap_or("doro"));
    let locs = [
        (29, 152), (29, 152), (29, 151), (29, 149), (29, 153), (29, 153), (29, 151), (29, 149),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let png = load_image(format!("dorowaimai/{i}.png"))?;
        let base = images[0].resize_exact((40, 30));
        let rotated = base.rotate(-15.0);
        let new_x = locs[i].0 + 20 - rotated.width() / 2;
        let new_y = locs[i].1 + 15 - rotated.height() / 2;
        let t = text_image(
            &text,
            25.0,
            197.0,
            text_params!(
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 255), 3.0),
            ),
        );
        let x = (png.width() - t.width()) / 2;
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&png, (0, 0), None);
        canvas.draw_image(&rotated, (new_x, new_y), None);
        canvas.draw_image(&t, (x, 3), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 8, duration: 0.04 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "dorowaimai",
    dorowaimai,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["doro"],
    keywords = &["doro外卖"],
    date_created = local_date(2025, 7, 4),
    date_modified = local_date(2025, 7, 4),
}
