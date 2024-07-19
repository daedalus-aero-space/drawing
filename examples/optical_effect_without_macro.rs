use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_svg::SVGOptions;
use palette::Srgba;
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
    let rectangle1 = Rectangle::default();

    // creates a grey font
    let mut rectangle1 = Style::new(rectangle1);

    rectangle1.width(15.);

    rectangle1.height(15.);

    rectangle1.fill(Srgba::new(0.588, 0.588, 0.588, 0.2));

    let rectangle1 = Shape::from(rectangle1);

    // creates a group
    let mut group = Group::default();

    group.shapes = vec![];

    // add rectangle1 in the group at first to let it be the font
    group.shapes.push(Shape::from(rectangle1));

    // creates the optical effect
    let optical_effect = ThickArc::default();

    for n in 0..11 {
        let mut optical_effect = Style::new(optical_effect.clone());

        optical_effect.inner_radius(0.);

        optical_effect.outer_radius(10.);

        optical_effect.span_angle(PI / 10_f32);

        // paints the inside of the thick arc in black
        optical_effect.fill(Srgba::new(0.0, 0.0, 0.0, 1.0));

        // chooses a rotation of (n*PI)/5 radians in the trigonometric direction
        optical_effect.rotate(Rotation2::new(PI * (n as f32) / 5_f32));

        // add the nth optical effect in the group
        group.shapes.push(Shape::from(optical_effect));
    }

    let rectangle2 = Rectangle::default();

    // creates a rectangle for the border
    let mut rectangle2 = Style::new(rectangle2);

    rectangle2.width(15.);

    rectangle2.height(15.);

    rectangle2.stroke(Stroke::new_full(Srgba::new(0.0, 0.0, 0.0, 0.01), 1.));

    // creates a white circle in the middle
    let circle = Circle::default().with_radius(1.);

    let mut circle = Style::new(circle);

    circle.fill(Srgba::new(1.0, 1.0, 1.0, 1.0));

    // transforms rectangle2 and circle into Shape
    let rectangle2 = Shape::from(rectangle2);
    let circle = Shape::from(circle);

    group.shapes.push(Shape::from(rectangle2));
    group.shapes.push(Shape::from(circle));

    // prints in svg version with Shape::from(...) -> Shape::Group(group) because of the group
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/optical_effect.svg"),
        dessin_svg::to_string_with_options(
            &Shape::Group(group),
            SVGOptions {
                viewport: dessin_svg::ViewPort::ManualCentered {
                    width: 14.,
                    height: 14.,
                },
            },
        )
        .unwrap(),
    )
    .unwrap();
}
//.to_svg_with_options(SVGOptions{viewport:dessin_svg::ViewPort::ManualCentered permits to choose how we will see the svg

// Note :
// (1) This solution is not be optimal because we can merge these two renctangles into one.
// (2) This code micht not return what we expect if you use "microsoft edge" but there is no same case with others like "google chrome"
//     or "firefox"
