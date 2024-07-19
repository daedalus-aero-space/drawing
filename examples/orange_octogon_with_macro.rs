//Attention ! It is the same way to make all polygons, you just have to replace : "Octogon" by "Polygon< the number of side you want >"

use dessin::{contrib::polygons::Octogon, nalgebra::Rotation2, prelude::*};
use palette::Srgba;
use project_root::get_project_root;
use std::fs;

fn main() {
    let octogon: Shape = dessin2!(
        Octogon!(
            // paints the inside of the octogon in bright orange
            fill = Srgba::new(1.0, 0.749, 0.0, 1.0),
            // We decide to not use stroke but it is possible :
            // stroke = (Stroke::new_dashed(Strba::new(0.0, 0.0, 0.0, 1.0),
            // width: 0.1,
            // on: 0.2,
            // off: 0.1
            // ),

            // chooses a rotation of -10 radians in the trigonometric direction
            rotate = Rotation2::new(-10_f32.to_radians())
        ) > ()
    );

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/orange_octogon.svg"),
        dessin_svg::to_string(&octogon).unwrap(),
    )
    .unwrap();
}
