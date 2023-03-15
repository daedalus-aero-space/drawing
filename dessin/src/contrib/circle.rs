use nalgebra::{Scale2, Transform2};

use crate::{prelude::Ellipse, Shape, ShapeOp};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Circle {
    pub local_transform: Transform2<f32>,
}

impl Circle {
    #[inline]
    pub fn radius(&mut self, radius: f32) -> &mut Self {
        self.resize(Scale2::new(radius, radius));
        self
    }
    #[inline]
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius(radius);
        self
    }
}

impl ShapeOp for Circle {
    #[inline]
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.local_transform *= transform_matrix;
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        &self.local_transform
    }
}

impl From<Circle> for Shape {
    #[inline]
    fn from(Circle { local_transform }: Circle) -> Self {
        Shape::Ellipse(Ellipse { local_transform })
    }
}
