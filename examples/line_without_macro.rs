use dessin::{nalgebra::Point2, prelude::*};
use project_root::get_project_root;
use std::fs;

fn main() {
    let circle_point = Circle::default().with_radius(0.1);
    let line = Line::default();

    let mut line = Style::new(line);

    line.from(Point2::new(1., 0.));

    line.to(Point2::new(12., 5.2));

    line.fill(rgb(255, 100, 100));

    line.stroke(Stroke::Full {
        color: rgb(255, 100, 100),
        width: 0.05,
    });

    line.translate([5., 1.]);

    let circle_point = Shape::from(circle_point);
    let line = Shape::from(line);

    let mut group = Group::default();
    group.shapes = vec![circle_point, line];

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/line.svg"),
        dessin_svg::to_string(&Shape::Group(group)).unwrap(),
    )
    .unwrap();
}
