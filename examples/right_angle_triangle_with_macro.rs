use dessin::{nalgebra::Rotation2, prelude::*};
use palette::{Srgb, Srgba};
use project_root::get_project_root;
use std::{f32::consts::PI, fs};

fn main() {
    let triangle: Shape = dessin2!([
        Triangle!(
            //chooses the size of the first side of the triangle which is on the x axis without rotation : 3
            width_x_axis = 3.,
            //chooses the size of the second side of the triangle : 4
            size_axis_angle = 4.,
            // chooses a right angle in radiant which is : PI/2 or 3PI/2
            angle = PI / 2.,
            // paints the inside of the triangle in green
            fill = Srgb::new(0.0, 0.0, 0.392),
            // creates a black pointing margin with a width of 0.1 (0.05 outside and the same inside the triangle), a length of 0.2 and
            // a space of 0.1 between each of them
            stroke = Stroke::new_dashed(Srgba::new(0.0, 0.0, 0.0, 0.2522115), 0.1, 0.2, 0.1),
            // chooses a rotation of 0 radians in the trigonometric direction
            rotate = Rotation2::new(0_f32.to_radians())
        ),
        //here, the hypotenuse should be 5
    ]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/right_angle_triangle.svg"),
        dessin_svg::to_string(&triangle).unwrap(),
    )
    .unwrap();
}
