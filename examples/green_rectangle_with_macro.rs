use dessin::{nalgebra::Rotation2, prelude::*};
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
    let rectangle: Shape = dessin2!([Rectangle!(
        // chooses a width of 11
        width = 11.,
        // chooses a height of 6
        height = 6.,
        // paints the inside of the rectangle in green
        fill = Srgb::new(0.0, 1.0, 0.0),
        // creates a grey margin of 0.2 (0.05 outside and the same inside the rectangle)
        stroke = Stroke::new_full(Srgb::new(0.576, 0.576, 0.576), 0.1),
        //chooses a rotation of 6 radians in the trigonometric direction
        rotate = Rotation2::new(6_f32.to_radians()),
    ),]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/green_rectangle.svg"),
        dessin_svg::to_string(&rectangle).unwrap(),
    )
    .unwrap();
}
