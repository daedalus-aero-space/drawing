use std::{fs, time::Duration};

use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_image::*;
use dessin_svg::*;
use palette::{named, Srgb};
use project_root::get_project_root;

fn main() {
    let skip_animation = std::env::var("NO_ANIMATION") == Ok("1".to_string());

    let path = get_project_root()
        .unwrap()
        .join("examples/out/animation.svg");

    let test_img = dessin2!(
        polygons::Triangle!(fill = Srgb::<f32>::from_format(named::BLUE).into_linear(),)
            > (scale = [50., 50.])
    )
    .rasterize()
    .unwrap();

    let triangle = Default::default();

    let frame = dessin2!(
        [
            Circle!(
                stroke = Stroke::new_full(Srgb::<f32>::from_format(named::RED).into_linear(), 0.5),
                radius = 5.
            ),
            Dynamic::<Image>(_ref = &triangle, image = test_img, scale = [3., 3.],),
        ] > (scale = [100., 100.])
    );

    loop {
        let final_image = to_string(&frame.clone()).unwrap();
        fs::write(&path, final_image).unwrap();

        if skip_animation {
            break;
        }

        std::thread::sleep(Duration::from_millis(100));
        let mut t = triangle.write().unwrap();
        t.rotate(Rotation2::new(0.3));
    }
}
