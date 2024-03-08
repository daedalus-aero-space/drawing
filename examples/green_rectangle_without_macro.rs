use dessin::{nalgebra::Rotation2, prelude::*};
use palette::Srgba;
use project_root::get_project_root;
use std::fs;

fn main() {
    // creates a rectangle with a width of 11 and a height of 6
    let rectangle = Rectangle::default().with_width(11.).with_height(6.);

    let mut rectangle = Style::new(rectangle);

    // paints the inside of the rectangle in green
    rectangle.fill(Srgba::new(0.0, 1.0, 0.0, 1.0));

    // creates a grey margin of 0.2 (0.05 outside and 0.05 inside the rectangle)
    rectangle.stroke(Stroke::Full {
        color: Srgba::new(0.376, 0.376, 0.376, 1.0),
        width: 0.1,
    });

    //chooses a rotation of 6 radians in the trigonometric direction
    rectangle.rotate(Rotation2::new(6_f32.to_radians()));

    //prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/green_rectangle.svg"),
        dessin_svg::to_string(&rectangle.into()).unwrap(),
    )
    .unwrap();
}
