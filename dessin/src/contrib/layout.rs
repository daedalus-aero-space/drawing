use crate::prelude::*;
use nalgebra::Transform2;

/// Display children on top of one another
#[derive(Debug, Default, Clone, Shape)]
pub struct VerticalLayout {
    ///
    #[local_transform]
    pub local_transform: Transform2<f32>,

    /// Shapes
    pub shapes: Vec<Shape>,

    #[shape(skip)]
    start_bottom: bool,

    /// Gap between each elements
    pub gap: f32,

    #[shape(skip)]
    metadata: Vec<(String, String)>,
}
impl VerticalLayout {
    /// In the case of a Group, local_transform is discarded as the shapes will be rearranged in a vertical layout
    #[inline]
    pub fn of<T: Into<Shape>>(&mut self, shape: T) -> &mut Self {
        match shape.into() {
            Shape::Group(Group {
                local_transform,
                shapes,
                metadata,
            }) => {
                self.metadata.extend(metadata);
                self.shapes.extend(shapes.into_iter().map(|mut v| {
                    v.transform(local_transform);
                    v
                }));
            }
            x => {
                self.shapes.push(x);
            }
        }

        self
    }

    /// Chained version of [`VerticalLayout::of`]
    #[inline]
    pub fn with<T: Into<Shape>>(mut self, shape: T) -> Self {
        self.of(shape);
        self
    }

    /// Iterator version of [`VerticalLayout::of`]
    #[inline]
    pub fn extend<T: IntoIterator<Item = Shape>>(&mut self, shapes: T) -> &mut Self {
        self.shapes.extend(shapes);
        self
    }

    /// Flow from bottom to top
    #[inline]
    pub fn start_from_bottom(&mut self) -> &mut Self {
        self.start_bottom = true;
        self
    }
    /// Flow from bottom to top
    #[inline]
    pub fn with_start_from_bottom(mut self) -> Self {
        self.start_from_bottom();
        self
    }

    /// Flow from top to bottom (default)
    #[inline]
    pub fn start_from_top(&mut self) -> &mut Self {
        self.start_bottom = false;
        self
    }
    /// Flow from top to bottom (default)
    #[inline]
    pub fn with_start_from_top(mut self) -> Self {
        self.start_from_top();
        self
    }
}

impl From<VerticalLayout> for Shape {
    fn from(
        VerticalLayout {
            local_transform,
            shapes,
            start_bottom,
            gap,
            metadata,
        }: VerticalLayout,
    ) -> Self {
        let mut y = 0.;

        dessin2!(
            for shape in (shapes) {
                let mut shape = shape;

                let bb = shape.local_bounding_box().into_straight();

                let y_translation = if start_bottom {
                    -bb.bottom() // Recenter the bottom of the shape to be 0
                + y // And then, above all the others
                } else {
                    -bb.top() // Recenter the top of the shape to be 0
                - y // And then, below all the others
                };

                shape.translate([0., y_translation]);

                y += bb.height() + gap;

                shape
            } > (transform = local_transform, extend_metadata = metadata)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use assert_float_eq::*;
    use nalgebra::Point2;

    #[test]
    fn one_element() {
        let layout = dessin2!(VerticalLayout(of = Text::default().with_font_size(10.)) > ());

        let bb: BoundingBox<UnParticular> = layout.local_bounding_box();

        assert_float_absolute_eq!(bb.height(), 10., 0.0001);
    }

    #[test]
    fn two_distinct_elements() {
        let layout = dessin2!(
            VerticalLayout(
                of = Text::default().with_font_size(10.),
                of = Text::default().with_font_size(10.),
            ) > ()
        );

        let bb: BoundingBox<UnParticular> = layout.local_bounding_box();

        assert_float_absolute_eq!(bb.height(), 20., 0.0001);
    }

    #[test]
    fn two_elements_vec_with_gap() {
        let layout = dessin2!(
            VerticalLayout(
                of = dessin2!([Text(font_size = 10.), Text(font_size = 10.)]),
                gap = 4.
            ) > ()
        );

        let bb: BoundingBox<UnParticular> = layout.local_bounding_box();

        assert_float_absolute_eq!(bb.height(), 24., 0.0001);
    }

    #[test]
    fn two_distinct_elements_with_gap() {
        let layout = dessin2!(
            VerticalLayout(
                of = Text::default().with_font_size(10.),
                of = Text::default().with_font_size(10.),
                gap = 4.,
            ) > ()
        );

        let bb: BoundingBox<UnParticular> = layout.local_bounding_box();

        assert_float_absolute_eq!(bb.height(), 24., 0.0001);
    }

    #[test]
    fn base_layout() {
        let layout = dessin2!(VerticalLayout(
            of = dessin2!([Circle(radius = 10.), Circle(radius = 10.),]),
        ));

        let Shape::Group(Group {
            local_transform: _,
            shapes,
            ..
        }) = Shape::from(layout)
        else {
            panic!("Not a group")
        };

        let [c1, c2] = shapes.as_slice() else {
            panic!("Expected 2 shapes, got {:#?}", shapes)
        };

        let p = Point2::new(0., 0.);
        assert_eq!(c1.local_transform() * p, Point2::new(0., -10.));
        assert_eq!(c2.local_transform() * p, Point2::new(0., -30.));
    }

    #[test]
    fn transformed_layout() {
        let layout = dessin2!(VerticalLayout(
            of = {
                dessin2!([
                    Circle(radius = 10., translate = [0., 5.]),
                    Circle(radius = 10., translate = [0., -10.]),
                    Circle(radius = 10., translate = [0., 0.]),
                ])
            }
        ));

        let Shape::Group(Group {
            local_transform: _,
            shapes,
            ..
        }) = Shape::from(layout)
        else {
            panic!("Not a group")
        };

        let [c1, c2, c3] = shapes.as_slice() else {
            panic!("Expected 3 shapes, get: {:#?}", shapes)
        };

        let p = Point2::new(0., 0.);
        assert_eq!(c1.local_transform() * p, Point2::new(0., -10.));
        assert_eq!(c2.local_transform() * p, Point2::new(0., -30.));
        assert_eq!(c3.local_transform() * p, Point2::new(0., -50.));
    }

    #[test]
    fn layout_of_polygons() {
        let height_triangle = polygons::Triangle::default()
            .as_shape()
            .local_bounding_box()
            .height();

        assert_float_absolute_eq!(height_triangle, 2. * (3f32.sqrt() / 2.), 10e-5);

        let shape = dessin2!([VerticalLayout(
            of = dessin2!(polygons::Triangle()),
            of = dessin2!(Circle(radius = 1.)),
        )]);

        let bb = shape.local_bounding_box();

        dbg!(shape);

        assert_eq!(bb.width(), 2.);
        assert_eq!(bb.height(), 2. + height_triangle);
    }

    use palette::{named, Srgb};
    #[test]
    fn layout_of_textbox() {
        let text = "test\nwhy\nnot";
        let gap = 2.;

        let first_text = dessin2!(TextBox!(
            { text },
            fill = Srgb::<f32>::from_format(named::BLACK).into_linear(),
            font_size = 3.6,
            align = TextAlign::Left,
            width = 115.,
            line_spacing = 2.,
        ));

        let layout = dessin2!(VerticalLayout(
            start_from_bottom,
            { gap },
            of = dessin2!([
                { first_text }(),
                Text!(
                    text = "Notes",
                    fill = Srgb::<f32>::from_format(named::BLACK).into_linear(),
                    font_weight = FontWeight::Bold,
                    font_size = 3.6,
                    align = TextAlign::Left,
                ),
            ]),
            translate = [-105. + 2., -148.5 + 5.]
        ));

        let shape = Shape::from(layout);

        let bounding_bb = shape.local_bounding_box();

        let height = bounding_bb.height();
        let min_y = bounding_bb.bottom_left().y;

        assert_float_absolute_eq!(height, 3. * gap + 3. * 3.6 + 3.6, 0.1);
        assert_float_absolute_eq!(min_y, -148.5 + 5., 0.1);
    }
}
