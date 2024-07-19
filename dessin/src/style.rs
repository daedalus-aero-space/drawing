use crate::prelude::*;
use nalgebra::{Rotation2, Scale2, Transform2, Translation2, Vector2};
use palette::{IntoColor, Srgba};
use std::{
    f32::consts::FRAC_1_SQRT_2,
    fmt,
    ops::{Deref, DerefMut, Mul},
};

/// Color
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {}

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct StylePosition {
    pub stroke: Option<Stroke>,
    pub fill: Option<Srgba>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stroke {
    Full {
        color: Srgba,
        width: f32,
    },
    Dashed {
        color: Srgba,
        width: f32,
        on: f32,
        off: f32,
    },
}

impl Stroke {
    pub fn new_full<F: IntoColor<Srgba>>(color: F, width: f32) -> Self {
        let color = color.into_color();
        Self::Full { color, width }
    }

    pub fn new_dashed<F: IntoColor<Srgba>>(color: F, width: f32, on: f32, off: f32) -> Self {
        let color = color.into_color();
        Self::Dashed {
            color,
            width,
            on,
            off,
        }
    }
}

impl Mul<Stroke> for Transform2<f32> {
    type Output = Stroke;
    fn mul(self, rhs: Stroke) -> Self::Output {
        match rhs {
            Stroke::Full { color, width } => Stroke::Full {
                color,
                width: (self * Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)).magnitude() * width,
            },
            Stroke::Dashed {
                color,
                width,
                on,
                off,
            } => {
                let factor = (self * Vector2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)).magnitude();

                Stroke::Dashed {
                    color,
                    width: width * factor,
                    on: on * factor,
                    off: off * factor,
                }
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Style<T> {
    pub shape: T,
    pub fill: Option<Srgba>,
    pub stroke: Option<Stroke>,
}
impl<T> Style<T> {
    #[inline]
    pub fn new(shape: T) -> Self {
        Style {
            shape,
            fill: None,
            stroke: None,
        }
    }

    #[inline]
    pub fn stroke<S: Into<Stroke>>(&mut self, stroke: S) -> &mut Self {
        self.stroke = Some(stroke.into());
        self
    }
    #[inline]
    pub fn with_stroke<S: Into<Stroke>>(mut self, stroke: S) -> Self {
        self.stroke(stroke);
        self
    }

    #[inline]
    pub fn fill<F: IntoColor<Srgba>>(&mut self, fill: F) -> &mut Self {
        self.fill = Some(fill.into_color());
        self
    }
    #[inline]
    pub fn with_fill<F: IntoColor<Srgba>>(mut self, fill: F) -> Self {
        self.fill(fill);
        self
    }
}

impl<T> Deref for Style<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Style<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T: Into<Shape>> From<Style<T>> for Shape {
    #[inline]
    fn from(
        Style {
            shape,
            fill,
            stroke,
        }: Style<T>,
    ) -> Self {
        if fill.is_none() && stroke.is_none() {
            shape.into()
        } else {
            Shape::Style {
                fill,
                stroke,
                shape: Box::new(shape.into()),
            }
        }
    }
}

impl<T: ShapeOp> ShapeOp for Style<T> {
    #[inline]
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.shape.transform(transform_matrix);
        self
    }

    #[inline]
    fn translate<U: Into<Translation2<f32>>>(&mut self, translation: U) -> &mut Self {
        self.shape.translate(translation);
        self
    }
    #[inline]
    fn scale<S: Into<Scale2<f32>>>(&mut self, scale: S) -> &mut Self {
        self.shape.scale(scale);
        self
    }
    #[inline]
    fn rotate<R: Into<Rotation2<f32>>>(&mut self, rotation: R) -> &mut Self {
        self.shape.rotate(rotation);
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        self.shape.local_transform()
    }
    #[inline]
    fn global_transform(&self, parent_transform: &Transform2<f32>) -> Transform2<f32> {
        self.shape.global_transform(parent_transform)
    }
}

impl<T: ShapeBoundingBox> ShapeBoundingBox for Style<T> {
    #[inline]
    fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
        self.shape.local_bounding_box()
    }
}
