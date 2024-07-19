use dessin::prelude::*;
use palette::{Srgb, Srgba};
use project_root::get_project_root;
use std::fs;

fn main() {
    // creates a circle with radius of 11
    let circle = Circle::default().with_radius(11.);

    let mut circle = Style::new(circle);

    // paints the inside of the circle in red
    circle.fill(Srgba::new(1.0, 0.0, 0.0, 1.0));

    // creates a grey margin of 0.2 (0.1 outside and 0.1 inside the circle)
    circle.stroke(Stroke::new_full(Srgba::new(0.576, 0.576, 0.576, 1.0), 0.2));

    // let circle = Style::new(circle)
    //     .with_fill(Srgb::new(1.0, 0.0, 0.0))
    //     .with_stroke(Stroke::new_full(Srgb::new(0.376, 0.376, 0.376), 0.2));

    //prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/red_circle.svg"),
        dessin_svg::to_string(&circle.into()).unwrap(),
    )
    .unwrap();
}
