use dessin::prelude::*;
use palette::Srgba;
use project_root::get_project_root;
use std::fs;

fn main() {
    let rectangle_1 = dessin2!(Rectangle!(
        width = 3.,
        height = 2.,
        translate = [1., 0.],
        fill = Srgba::new(1.0, 0.0, 1.0, 1.0)
    ));

    let base = dessin2!(Padding<Shape>( // here, we can replace 'Shape' with 'Rectangle' but in case we want to use the
                                                        // Padding to a multiple geometric form, using Shape become a must
        shape = rectangle_1.clone(),
        padding_left = 1.5,
        padding_right = 1.,
        padding_top = 0.8,
        padding_bottom = 1.,

    ));

    let rectangle_2 = dessin2!(Rectangle!(
        width = 5.5,
        height = 3.8,
        stroke = Stroke::new_full(Srgba::new(0.0, 0.7, 0.0, 1.0), 0.2),
        translate = [0.75, -0.1]
    ));

    let base = Shape::from(base);
    let rectangle_1 = Shape::from(rectangle_1);
    let rectangle_2 = Shape::from(rectangle_2);

    // creates a group
    let mut group = Group::default();

    group.shapes = vec![];

    group.shapes.push(base);
    group.shapes.push(rectangle_1);
    group.shapes.push(rectangle_2);

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/padding.svg"),
        dessin_svg::to_string(&Shape::Group(group)).unwrap(),
    )
    .unwrap();
}
