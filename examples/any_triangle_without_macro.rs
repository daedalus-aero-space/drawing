use std::fs;

use dessin::{nalgebra::Rotation2, prelude::*};
use palette::Srgb;
use project_root::get_project_root;

fn main() {
    // creates a triangle
    let triangle = Triangle::default();

    let mut triangle = Style::new(triangle);

    // chooses the size of the first side of the triangle which is on the x axis without rotation : 4
    triangle.width_x_axis(4.);

    // chooses the size of the second side of the triangle : 12
    triangle.size_axis_angle(12.);

    // chooses an angle of 0.5
    triangle.angle(0.5);

    // paints the inside of the triangle in bright pink
    triangle.fill(Srgb::new(1.0, 0.0, 0.498)); // here, ".into()" is not needed

    // creates a black margin of 0.1 (0.05 outside and 0.05 inside the triangle)
    triangle.stroke(Stroke::new_dashed(Srgb::new(0.0, 0.0, 0.0), 0.1, 0.2, 0.1));

    // chooses a rotation of -10 radians in the trigonometric direction
    triangle.rotate(Rotation2::new(-10_f32.to_radians()));

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/any_triangle.svg"),
        dessin_svg::to_string(&triangle.into()).unwrap(),
    )
    .unwrap();
}
