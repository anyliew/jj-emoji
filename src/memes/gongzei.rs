use skia_safe::{Color, Image, ISize};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    text::{Text2Image, TextParams},
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct GongzeiOptions {
    #[option(short, long, description = "指定名字")]
    name: Option<String>,
}

fn text_image(text: &str, font_size: f32, wrap: f32, params: TextParams) -> Image {
    let mut t2i = Text2Image::from_text(text, font_size, params);
    t2i.layout(wrap);
    let w = t2i.longest_line().ceil() as i32;
    let h = t2i.height().ceil() as i32;
    let mut surface = new_surface(ISize::new(w.max(1), h.max(1)));
    t2i.draw_on_canvas(surface.canvas(), (0, 0));
    surface.image_snapshot()
}

fn gongzei(images: Vec<InputImage>, texts: Vec<String>, options: GongzeiOptions) -> Result<Vec<u8>, Error> {
    let mut name = options.name.clone().unwrap_or_else(|| "工贼".to_string());
    if name.chars().count() > 8 {
        name = name.chars().take(8).collect();
    }
    let text = texts.first().map(|s| s.as_str()).unwrap_or("我爱加班");
    let head_locs = [(64, 60), (54, 55), (41, 55), (55, 59)];
    let name_locs = [(225, 242), (225, 240), (225, 242), (225, 240)];

    let func = |i: usize, images: Vec<Image>| {
        let background = load_image("gongzei/dengzi.png")?;
        let img = images[0].resize_exact((164, 164)).circle();
        let png = load_image(format!("gongzei/{i}.png"))?;
        let t = text_image(
            text,
            42.0,
            440.0,
            text_params!(
                font_families = &["033-SSFangTangTi"],
                paint = new_paint(Color::from_rgb(0, 0, 0)),
            ),
        );
        let name_img = text_image(
            &name,
            18.0,
            200.0,
            text_params!(
                paint = new_paint(Color::from_rgb(27, 27, 27)),
            ),
        );
        let name_rot = name_img.rotate(-7.0);
        let tx = (background.width() - t.width()) / 2;
        let nx = name_locs[i].0 - name_rot.width() / 2;
        let ny = name_locs[i].1 - name_rot.height() / 2;
        let mut surface = new_surface(background.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&background, (0, 0), None);
        canvas.draw_image(&img, head_locs[i], None);
        canvas.draw_image(&png, (0, 0), None);
        canvas.draw_image(&t, (tx, 258), None);
        canvas.draw_image(&name_rot, (nx, ny), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo { frame_num: 4, duration: 0.08 },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "gongzei",
    gongzei,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我爱加班"],
    keywords = &["工贼"],
    date_created = local_date(2025, 5, 17),
    date_modified = local_date(2025, 5, 17),
}
