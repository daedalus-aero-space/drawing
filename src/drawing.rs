use crate::{
    position::Rect,
    shapes::{
        arc::Arc, circle::Circle, drawing::EmbeddedDrawing, image::Image, line::Line, text::Text,
        Shape, ShapeType,
    },
    Size,
};
use algebra::{vec2, Vec2};

pub trait AddShape<T> {
    fn add(&mut self, shape: T) -> &mut Self;
}

/// Drawing is a collection of shapes.
/// ```
/// # use drawing::{
/// #     Drawing,
/// #     AddShape,
/// #     shape::{
/// #         Text,
/// #         Line,
/// #         Circle,
/// #         Arc,
/// #         { Image, ImageFormat },
/// #     },
/// #     vec2,
/// #     Angle,
/// # };
///
/// let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
///
/// drawing.add(
///         Text::new("Hello World".to_owned())
///             .at(vec2(50., 50.))
///     )
///     .add(
///         Line::from(vec2(0., 0.)).to(vec2(100., 100.))
///     )
///     //.add(
///     //    Circle::at(vec2(50., 50.)).with_radius(10.)
///     //)
///     .add(
///         Arc::new()
///             .at(vec2(50., 50.))
///             .with_inner_radius(10.)
///             .with_outer_radius(20.)
///             .with_start_angle(Angle::deg(0.))
///             .with_end_angle(Angle::deg(180.))
///     )
///     .add(
///         Image::new(ImageFormat::PNG(include_bytes!("../rustacean-flat-happy.png").to_vec()))
///             .at(vec2(50., 50.))
///             .with_size(vec2(10., 10.))
///     );
///     
///     //let other_drawing = Drawing::empty()
///     //    .with_canvas_size((210., 297.))
///     //    .add(
///     //        EmbeddedDrawing::from(drawing)
///     //            .at(vec2(100., 100.))
///     //            .with_size(vec2(10., 10.))
///     //    );
/// ```
#[derive(Debug, Clone)]
pub struct Drawing {
    pub canvas_size: Size,
    pub(crate) shapes: Vec<Shape>,
}
impl Drawing {
    /// Default constructor, creates an empty drawing.
    pub const fn empty() -> Self {
        Drawing {
            canvas_size: vec2(0., 0.),
            shapes: vec![],
        }
    }

    pub const fn with_canvas_size(mut self, canvas_size: Vec2) -> Self {
        self.canvas_size = canvas_size;
        self
    }

    /// Get access to this drawing's shapes.
    /// ```
    /// # use drawing::{
    /// #     Drawing,
    /// #     AddShape,
    /// #     shape::{
    /// #         Text,
    /// #         Line,
    /// #         Circle,
    /// #         Arc,
    /// #         { Image, ImageFormat },
    /// #     },
    /// #     vec2,
    /// # };
    ///
    /// let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));
    ///
    /// drawing.add(
    ///         Text::new("Hello World".to_owned())
    ///             .at(vec2(50., 50.))
    ///     )
    ///     .add(
    ///         Line::from(vec2(0., 0.)).to(vec2(100., 100.))
    ///     );
    ///
    /// let shapes = drawing.shapes();
    /// dbg!("{:?}", shapes);
    /// ```
    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }

    pub fn into_embedded(self) -> EmbeddedDrawing {
        EmbeddedDrawing::from_drawing(self)
    }
}

impl AddShape<Text> for Drawing {
    fn add(&mut self, shape: Text) -> &mut Self {
        self.shapes.push(Shape {
            pos: shape.pos,
            style: shape.style,
            shape_type: ShapeType::Text {
                text: shape.text,
                align: shape.align,
                font_size: shape.font_size,
                font_weight: shape.font_weight,
            },
        });
        self
    }
}
impl AddShape<Line> for Drawing {
    fn add(&mut self, shape: Line) -> &mut Self {
        let pos = Rect::new()
            .at((shape.from + shape.to) / 2.)
            .with_size((shape.from - shape.to).abs());

        self.shapes.push(Shape {
            pos,
            style: shape.style,
            shape_type: ShapeType::Line {
                from: shape.from,
                to: shape.to,
            },
        });
        self
    }
}
// impl AddShape<Circle> for Drawing {
//     fn add(&mut self, shape: Circle) -> &mut Self{
//         self.shapes.push(Shape::Circle(shape));
//         self
//     }
// }
impl AddShape<Arc> for Drawing {
    fn add(&mut self, shape: Arc) -> &mut Self {
        self.shapes.push(Shape {
            pos: shape.pos,
            style: shape.style,
            shape_type: ShapeType::Arc {
                inner_radius: shape.inner_radius,
                outer_radius: shape.outer_radius,
                start_angle: shape.start_angle,
                end_angle: shape.end_angle,
            },
        });
        self
    }
}
impl AddShape<Image> for Drawing {
    fn add(&mut self, shape: Image) -> &mut Self {
        self.shapes.push(Shape {
            pos: shape.pos,
            style: shape.style,
            shape_type: ShapeType::Image { data: shape.data },
        });
        self
    }
}
// impl AddShape<EmbeddedDrawing> for Drawing {
//     fn add(
//         &mut self,
//         EmbeddedDrawing {
//             mut shapes,
//             pos,
//             canvas_anchor,
//             scale,
//         }: EmbeddedDrawing,
//     ) {
//         if canvas_anchor != Vec2::from_cartesian(0., 0.) {
//             unimplemented!()
//         }

//         shapes
//             .iter_mut()
//             .for_each(|s| s.apply_transform(pos, scale));
//         self.shapes.push(Shape::Drawing(shapes));
//     }
// }
