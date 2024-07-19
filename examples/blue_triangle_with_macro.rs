use dessin::{
    nalgebra::{Rotation2, Scale2},
    prelude::{polygons::Triangle, *},
};
use palette::Srgb;
use project_root::get_project_root;
use std::fs;

fn main() {
    let triangle: Shape = dessin2!([Triangle!(
        // chooses an equilateral triangle [(x,x) => equilateral] with a size of 5. [if you want an isosceles triangle : (x,y)]
        scale = Scale2::new(5., 5.),
        // paints the inside of the triangle in green
        fill = Srgb::new(0.0, 0.0, 1.0),
        // creates a black margin of 0.1 (0.05 outside and the same inside the triangle)
        stroke = Stroke::new_full(Srgb::new(0.0, 0.0, 0.0), 0.1),
        //chooses a rotation of 0 radians in the trigonometric direction
        rotate = Rotation2::new(0_f32.to_radians()),
    ),]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/blue_triangle.svg"),
        dessin_svg::to_string(&triangle).unwrap(),
    )
    .unwrap();
}
