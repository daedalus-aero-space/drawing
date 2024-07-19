use palette::Srgba;
use std::{f32::consts::PI, fs};

use dessin::{nalgebra::Rotation2, prelude::*};
use project_root::get_project_root;

fn main() {
    let basque_cross: Shape = dessin2!([
        // creates a little circle which serves as references for mouvements
        Circle!(radius = 0.01, fill = Srgba::new(1.0, 0.0, 0.0, 1.0),),
        for n in 0..=4 {
            dessin2!([
                // creates large half red circle
                ThickArc!(
                    // it starts at an angle of 90°
                    start_angle = PI / 2_f32,
                    outer_radius = 20.,
                    inner_radius = 0.,
                    span_angle = PI,
                    //here, alpha is not needed (= 1.0), so we can replace Srgba by Srgb (without alpha)
                    fill = Srgba::new(1.0, 0.0, 0.0, 1.0),
                    translate = [0., 20.],
                    // it rotates of 90° each time
                    rotate = Rotation2::new(PI * (n as f32) / 2_f32)
                ),
                // add a small red circle to the second part of the half large circle
                Circle!(
                    radius = 10.,
                    fill = Srgba::new(1.0, 0.0, 0.0, 1.0),
                    translate = [0., 30.],
                    // it rotates of 90° each time
                    rotate = Rotation2::new(PI * (n as f32) / 2_f32)
                ),
                //add a small white half circle to the first part of the half large circle
                ThickArc!(
                    // it starts at an angle of 90°
                    start_angle = PI / 2_f32,
                    outer_radius = 10.,
                    inner_radius = 0.,
                    span_angle = PI,
                    fill = Srgba::new(1.0, 1.0, 1.0, 1.0),
                    translate = [0., 10.],
                    // it rotates of 90° each time
                    rotate = Rotation2::new(PI * (n as f32) / 2_f32)
                )
            ])
        }
    ]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/basque_cross.svg"),
        dessin_svg::to_string(&basque_cross).unwrap(),
    )
    .unwrap();
}
