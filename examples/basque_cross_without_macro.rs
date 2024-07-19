use dessin::{nalgebra::Rotation2, prelude::*};
use palette::Srgba;
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
    let circle_point = Circle::default().with_radius(0.01);

    let large_half_circle = ThickArc::default();
    let small_red_circle = Circle::default();
    let little_half_circle = ThickArc::default();

    // creates a group
    let mut group = Group::default();

    group.shapes = vec![];
    let circle_point = Shape::from(circle_point);
    group.shapes.push(circle_point);
    for n in 0..=4 {
        // creates a large half red circle
        let mut large_half_circle = Style::new(large_half_circle.clone());

        large_half_circle.start_angle(PI / 2_f32);

        large_half_circle.outer_radius(20.);

        large_half_circle.inner_radius(0.);

        large_half_circle.span_angle(PI);

        //here, alpha is not needed (= 1.0), so we can replace Srgba by Srgb (without alpha)
        large_half_circle.fill(Srgba::new(1.0, 0.0, 0.0, 1.0));

        large_half_circle.translate([0., 20.]);

        large_half_circle.rotate(Rotation2::new(PI * (n as f32) / 2_f32));

        // add a small red circle to the second part of the half large circle
        let mut small_red_circle = Style::new(small_red_circle.clone());

        small_red_circle.radius(10.);

        small_red_circle.fill(Srgba::new(1.0, 0.0, 0.0, 1.0));

        small_red_circle.translate([0., 30.]);

        small_red_circle.rotate(Rotation2::new(PI * (n as f32) / 2_f32));

        // creates a little half white circle
        let mut little_half_circle = Style::new(little_half_circle.clone());

        little_half_circle.start_angle(PI / 2_f32);

        little_half_circle.outer_radius(10.);

        little_half_circle.inner_radius(0.);

        little_half_circle.span_angle(PI);

        little_half_circle.fill(Srgba::new(1.0, 1.0, 1.0, 1.0));

        little_half_circle.translate([0., 10.]);

        little_half_circle.rotate(Rotation2::new(PI * (n as f32) / 2_f32));

        let large_half_circle = Shape::from(large_half_circle);
        let small_red_circle = Shape::from(small_red_circle);
        let little_half_circle = Shape::from(little_half_circle);

        // add each figures in the group

        group.shapes.push(large_half_circle);
        group.shapes.push(small_red_circle);
        group.shapes.push(little_half_circle);
    }

    // prints in svg version with Shape::from(...) -> Shape::Group(group) because of the group
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/basque_cross.svg"),
        dessin_svg::to_string(&Shape::Group(group)).unwrap(),
    )
    .unwrap();
}
