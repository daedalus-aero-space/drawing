use dessin::{nalgebra::Rotation2, prelude::*};
use palette::{Srgb, Srgba};
use project_root::get_project_root;
use std::fs;

fn main() {
    let text: Shape = dessin2!([TextBox!(
        font_size = 5.,
        line_spacing = 1.,
        text = "Here we write some text",
        width = 20.,
        height = 10.,
        font_weight = FontWeight::Italic,
        // chooses centered vertical allign
        vertical_align = TextVerticalAlign::Center,
        // selects to align the beginning of the text on the left
        align = TextAlign::Left,
        // paints the inside of the text in bright orange
        fill = Srgba::new(1.0, 0.749, 0.0, 1.0),
        // We decide to not use stroke but it is possible
        stroke = Stroke::new_full(Srgb::new(0.588, 0.039, 0.039), 0.1),
        // chooses a rotation of 6 radians in the trigonometric direction
        rotate = Rotation2::new(6_f32.to_radians())
    ),]);

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/text.svg"),
        dessin_svg::to_string(&text).unwrap(),
    )
    .unwrap();
}
