use dessin::{nalgebra::Rotation2, prelude::*};
use palette::Srgba;
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
    let thick_arc: Shape = dessin2!([ThickArc!(
        // chooses a radius of 10 for the outer curve
        outer_radius = 10.,
        // chooses a radius of 5 for the inner curve
        inner_radius = 5.,
        // chooses an angle of PI to show the area of the thick arc (which depends of the 2 curve and this angle)
        span_angle = PI,
        // paints the inside of the thick arc in yellow
        fill = Srgba::new(1.0, 1.0, 0.0, 1.0),
        // creates a black margin of 0.2 (0.05 outside and the same inside the thick arc)
        stroke = Stroke::new_full(Srgba::new(0.0, 0.0, 0.0, 0.5), 0.5),
        // chooses a rotation of Pi/3 in radians in the trigonometric direction
        rotate = Rotation2::new(PI / 3_f32.to_radians())
    ),]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/yellow_thick_arc.svg"),
        dessin_svg::to_string(&thick_arc).unwrap(),
    )
    .unwrap();
}
