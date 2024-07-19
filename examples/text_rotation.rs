use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_image::ToImage;
use dessin_pdf::ToPDF;
use palette::{named, Srgb};
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

#[derive(Shape, Default)]
struct RotatedText {
    #[shape(into)]
    text: String,
    rotation: f32,
}
impl From<RotatedText> for Shape {
    fn from(RotatedText { text, rotation }: RotatedText) -> Self {
        let text = dessin2!(Text!(
            fill = Srgb::<f32>::from_format(named::BLACK).into_linear(),
            font_size = 1.,
            align = TextAlign::Center,
            vertical_align = TextVerticalAlign::Top,
            { text },
        ));

        let bb = text.local_bounding_box();
        let width = bb.width();
        let height = bb.height();

        dessin2!(
            [
                Rectangle!(
                    { width },
                    { height },
                    stroke =
                        Stroke::new_full(Srgb::<f32>::from_format(named::BLACK).into_linear(), 0.1)
                ),
                { text },
            ] > (translate = [0., 15.], rotate = Rotation2::new(rotation),)
        )
        .into()
    }
}

fn main() {
    let dessin = dessin2!(
        for (idx, text) in "Hello world! This is me!".split(" ").enumerate() {
            dessin2!(RotatedText(rotation = idx as f32 * -PI / 4., { text }))
        }
    );

    let path = get_project_root().unwrap().join("examples/out/");

    // SVG
    fs::write(
        path.join("text_rotation.svg"),
        dessin_svg::to_string(&dessin.clone()).unwrap(),
    )
    .unwrap();

    // Image
    dessin2!({ dessin }(scale = [5., 5.]))
        .rasterize()
        .unwrap()
        .into_rgba8()
        .save(path.join("text_rotation.png"))
        .unwrap();
}
