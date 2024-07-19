use dessin::{nalgebra::Point2, prelude::*};
use palette::Srgba;
use project_root::get_project_root;
use std::fs;

fn main() {
    let line: Shape = dessin2!([
        // creates a little circle as reference for a movement
        Circle!(radius = 0.1),
        // creates a line
        Line!(
            // chooses the starting point of the line
            from = Point2::new(1., 0.),
            // chooses the ending point of the line
            to = Point2::new(12., 5.2),
            // not needed here
            fill = Srgba::new(1.0, 0.392, 0.392, 1.0),
            stroke = Stroke::new_full(Srgba::new(1.0, 0.392, 0.392, 1.0), 0.05),
            translate = [5., 1.]
        )
    ]);

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/line.svg"),
        dessin_svg::to_string(&line).unwrap(),
    )
    .unwrap();
}
