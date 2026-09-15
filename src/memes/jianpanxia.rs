use skia_safe::{Color, Image, ISize};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    text::{Text2Image, TextParams},
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
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

fn jianpanxia(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let func = |i: usize, _: Vec<Image>| {
        let png = load_image(format!("jianpanxia/{i}.png"))?;
        let mut surface = new_surface(png.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&png, (0, 0), None);
        if i >= 41 {
            let t = text_image(
                text,
                20.0,
                296.0,
                text_params!(
                    paint = new_paint(Color::from_rgb(0, 0, 0)),
                ),
            );
            let x = (png.width() - t.width()) / 2;
            canvas.draw_image(&t, (x, 98), None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 50, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "jianpanxia",
    jianpanxia,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["来点涩图"],
    keywords = &["键盘侠"],
    date_created = local_date(2025, 9, 15),
    date_modified = local_date(2025, 9, 15),
}
