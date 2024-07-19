use dessin::{nalgebra::Rotation2, prelude::*};
use palette::Srgb;
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
    // creates a triangle
    let arc = Arc::default();

    let mut arc = Style::new(arc);

    arc.start_angle(0.);

    arc.end_angle(PI / 4.);

    // creates a margin of 0.1
    arc.stroke(Stroke::new_full(Srgb::new(0.0, 0.196, 0.294), 0.1));

    // chooses a rotation of -10 radians in the trigonometric direction
    arc.rotate(Rotation2::new(-10_f32.to_radians()));

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/arc.svg"),
        dessin_svg::to_string(&arc.into()).unwrap(),
    )
    .unwrap();
}
