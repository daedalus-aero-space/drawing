use dessin::prelude::*;
use palette::Srgba;
use std::fs;

use dessin::nalgebra::Rotation2;
use project_root::get_project_root;

fn main() {
    // here we use the circle as a point to have a reference to use when moving the diamond
    let circle = Circle::default().with_radius(0.1);

    // creates a diamond
    let diamond = Diamond::default();

    let mut diamond = Style::new(diamond);

    // chooses a width of 4 for following the x axis
    diamond.width(4.);

    // chooses a size of 5 between the origin and the diamond top apex following the y axis
    diamond.height_top(5.);

    // chooses a size of 3 between the origin and the diamond bottom apex following the y axis
    diamond.height_bottom(3.);

    // paints the inside of the diamond in diamond color
    diamond.fill(Srgba::new(0.746, 0.949, 1.0, 0.99));

    // creates a black margin of 0.1 (0.05 outside and 0.05 inside the diamond)
    diamond.stroke(Stroke::new_full(Srgba::new(0.0, 0.0, 0.0, 1.0), 0.1));

    // chooses a rotation of -10 radians in the trigonometric direction
    diamond.rotate(Rotation2::new(-10_f32.to_radians()));

    // moves of 15 following the x axis and 5 following the y axis
    diamond.translate([15., 5.]);

    // transforms circle and diamond into Shape
    let circle = Shape::from(circle);
    let diamond = Shape::from(diamond);

    // creates a group with diamond and circle
    let mut group = Group::default();

    group.shapes = vec![diamond, circle];

    // prints in svg version with Shape::from(...) -> Shape::Group(group) because of the group
    fs::write(
        get_project_root().unwrap().join("examples/out/diamond.svg"),
        dessin_svg::to_string(&Shape::Group(group)).unwrap(),
    )
    .unwrap();
}
