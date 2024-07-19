use dessin::{nalgebra::Rotation2, prelude::*};
use palette::{Srgb, Srgba};
use project_root::get_project_root;
use std::fs;

fn main() {
    let circle: Shape = dessin2!([Circle!(
        // chooses a radius of 11
        radius = 11., //11. is like a proportion of the box allowed
        // paints the inside of the circle in red
        fill = Srgba::new(1.0, 0.0, 0.0, 1.0),
        // creates a grey margin of 0.2 (0.1 outside and 0.1 inside the circle)
        stroke = Stroke::new_full(Srgb::new(0.576, 0.576, 0.576), 0.2),
        rotate = Rotation2::new(0_f32.to_radians()), //not visible yet but it's possible to see it in some conditions
    ),]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/red_circle.svg"),
        dessin_svg::to_string(&circle).unwrap(),
    )
    .unwrap();
}
