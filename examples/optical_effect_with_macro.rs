use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_svg::SVGOptions;
use palette::Srgba;
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
    let optical_effect: Shape = dessin2!([
        for n in 0..11 {
            dessin2!([ThickArc!(
                outer_radius = 10.,
                inner_radius = 0.,
                span_angle = PI / 10_f32,
                fill = Srgba::new(0.0, 0.0, 0.0, 1.0),
                // chooses a rotation of (n*PI)/5 radians in the trigonometric direction
                rotate = Rotation2::new(PI * (n as f32) / 5_f32)
            )])
        },
        Circle!(
            // creates a white circle in the middle
            radius = 1.,
            fill = Srgba::new(1.0, 1.0, 1.0, 1.0),
        ),
        Rectangle!(
            width = 15.,
            height = 15.,
            stroke = Stroke::new_full(Srgba::new(0.0, 0.0, 0.0, 0.01), 1.)
        )
    ]);

    let fond = optical_effect.local_bounding_box();

    // dbg!(fond.width()); // if we want to know the fond.width size

    //Here we want to create a grey font behind all
    let truc = dessin2!([
        Rectangle!(
            width = fond.width(),
            height = fond.height(),
            fill = Srgba::new(0.588, 0.588, 0.588, 0.2),
        ),
        // Add optical_effect before the new Rectangle
        { optical_effect }
    ]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/optical_effect.svg"),
        dessin_svg::to_string_with_options(
            &truc,
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

// there is a slight difference between the optical effect with and without the macro, but it's not certain that you'll see it
