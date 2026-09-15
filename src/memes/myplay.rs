use skia_safe::{Color, ISize, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct MyplayOptions {
    #[option(short, long, description = "指定名字")]
    name: Option<String>,
}

fn myplay(images: Vec<InputImage>, texts: Vec<String>, options: MyplayOptions) -> Result<Vec<u8>, Error> {
    let name = options
        .name
        .clone()
        .unwrap_or_else(|| "智商-1".to_string());
    let mut name_t2i = Text2Image::from_text(
        &name,
        18.0,
        text_params!(paint = new_paint(Color::from_rgb(27, 27, 27))),
    );
    name_t2i.layout(10000.0);
    let name_w = name_t2i.longest_line().ceil() as i32;
    let name_h = name_t2i.height().ceil() as i32;
    let mut name_surface = new_surface(ISize::new(name_w.max(1), name_h.max(1)));
    name_t2i.draw_on_canvas(name_surface.canvas(), (0, 0));
    let name_img = name_surface.image_snapshot();
    let text = if texts.is_empty() { "笨死了" } else { &texts[0] };
    let head_locs = [
        (48, 208), (48, 211), (48, 274), (48, 264), (48, 242), (48, 232),
        (48, 221), (48, 209), (48, 208), (48, 272), (48, 245), (48, 240),
        (48, 225), (48, 216), (48, 208), (48, 205),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let bg = load_image(format!("myplay/{i}.png"))?;
        let head = images[0].resize_exact((100, 100)).circle();
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        let (x, y) = head_locs[i];
        canvas.draw_image(&head, (x, y), None);
        canvas.draw_image(&bg, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 265, bg.width(), 765),
            text,
            35.0,
            35.0,
            text_params!(
                font_families = &["033-SSFangTangTi"],
                text_align = TextAlign::Center,
                paint = new_paint(Color::from_rgb(0, 0, 0)),
                stroke_paint = new_stroke_paint(Color::from_rgb(255, 255, 255), 3.0),
            ),
        )?;
        if i < 4 {
            let ny = 172 - i as i32 * 40;
            canvas.draw_image(&name_img, (90 - name_w / 2, ny - name_h / 2), None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 16, duration: 0.05 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "myplay",
    myplay,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["笨死了"],
    keywords = &["我敲"],
    date_created = local_date(2025, 5, 17),
    date_modified = local_date(2025, 5, 17),
}
