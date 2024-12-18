/// Font storage
pub mod font;

use crate::prelude::*;
use font::FontRef;
use na::{Point2, Unit, Vector2};
use nalgebra::{self as na, Transform2};

pub(crate) fn size_of(font: &fontdue::Font, s: &str, font_size: f32) -> f32 {
    s.chars()
        .scan(None, |last, curr| {
            let l = last.unwrap_or(' ');
            let r = if let Some(v) = font.horizontal_kern(l, curr, font_size) {
                v
            } else {
                font.metrics(curr, font_size).advance_width
            };

            *last = Some(curr);

            Some(r)
        })
        .sum()
}

/// Weight of a font
#[derive(Default, Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum FontWeight {
    #[default]
    /// Regular
    Regular,
    /// Bold
    Bold,
    /// Italic
    Italic,
    /// BoldItalic
    BoldItalic,
}

/// TextAlign
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    #[default]
    /// Left
    Left,
    /// Center
    Center,
    /// Right
    Right,
}

/// TextVerticalAlign
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TextVerticalAlign {
    #[default]
    /// Bottom
    Bottom,
    /// Center
    Center,
    /// Top
    Top,
}

pub struct TextPosition<'a> {
    pub text: &'a str,
    pub align: TextAlign,
    pub font_weight: FontWeight,
    pub on_curve: Option<CurvePosition>,
    pub font_size: f32,
    pub reference_start: Point2<f32>,
    pub direction: Unit<Vector2<f32>>,
    pub font: &'a Option<FontRef>,
}

#[derive(Debug, Clone, PartialEq, Shape)]
pub struct Text {
    /// [`ShapeOp`]
    #[local_transform]
    pub local_transform: Transform2<f32>,

    #[shape(into)]
    pub text: String,

    pub align: TextAlign,

    pub vertical_align: TextVerticalAlign,

    pub font_weight: FontWeight,

    #[shape(into_some)]
    pub on_curve: Option<Curve>,

    pub font_size: f32,

    #[shape(into_some)]
    pub font: Option<FontRef>,
}
impl Default for Text {
    fn default() -> Self {
        Text {
            text: Default::default(),
            local_transform: Default::default(),
            align: Default::default(),
            vertical_align: Default::default(),
            font_weight: Default::default(),
            on_curve: Default::default(),
            font_size: 10.,
            font: Default::default(),
        }
    }
}
impl Text {
    #[inline]
    pub fn maybe_font<F: Into<FontRef>>(&mut self, font: Option<F>) -> &mut Self {
        self.font = font.map(Into::into).into();
        self
    }
    #[inline]
    pub fn with_maybe_font<F: Into<FontRef>>(mut self, font: Option<F>) -> Self {
        self.maybe_font(font);
        self
    }

    pub fn position(&self, parent_transform: &Transform2<f32>) -> TextPosition {
        let transform = self.global_transform(parent_transform);

        let font_size = self.font_size * (transform * Vector2::new(0., 1.)).magnitude();
        let reference_start = transform
            * Point2::new(
                0.,
                match self.vertical_align {
                    TextVerticalAlign::Bottom => font_size / 2.,
                    TextVerticalAlign::Center => 0.,
                    TextVerticalAlign::Top => -font_size / 2.,
                },
            );

        TextPosition {
            text: &self.text,
            align: self.align,
            font_weight: self.font_weight,
            on_curve: self.on_curve.as_ref().map(|v| v.position(&transform)),
            font_size,
            reference_start,
            direction: Unit::new_normalize(transform * Vector2::new(1., 0.)),
            font: &self.font,
        }
    }
}

impl From<Text> for Shape {
    fn from(v: Text) -> Self {
        Shape::Text(v)
    }
}

impl ShapeBoundingBox for Text {
    fn local_bounding_box(&self) -> BoundingBox<UnParticular> {
        let fonts = crate::font::get(self.font.clone().unwrap_or_default());
        let raw_font = match fonts.get(FontWeight::Regular) {
            crate::font::Font::OTF(bytes) => bytes,
            crate::font::Font::TTF(bytes) => bytes,
        };

        let font = fontdue::Font::from_bytes(raw_font.as_slice(), fontdue::FontSettings::default())
            .unwrap();

        let width = size_of(&font, &self.text, self.font_size);

        BoundingBox::centered([width, self.font_size])
            .as_unparticular()
            .transform(self.local_transform())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        export::{Export, Exporter},
        prelude::*,
    };
    use nalgebra::{Point2, Rotation2};
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

    #[test]
    fn rotate_group() {
        let dessin = dessin2!(
            [
                Circle(translate = [0., 25.]),
                Text(
                    text = "1",
                    font_size = 30.,
                    vertical_align = TextVerticalAlign::Center,
                    translate = [0., 25.],
                ),
                Text(
                    text = "2",
                    font_size = 40.,
                    vertical_align = TextVerticalAlign::Center,
                    translate = [0., 0.]
                ),
                Text(
                    text = "3",
                    font_size = 15.,
                    vertical_align = TextVerticalAlign::Center,
                    translate = [0., -30.]
                ),
            ] > (rotate = Rotation2::new(FRAC_PI_4))
        );

        struct Exp;
        impl Exporter for Exp {
            type Error = ();

            fn start_style(&mut self, _style: StylePosition) -> Result<(), Self::Error> {
                Ok(())
            }

            fn end_style(&mut self) -> Result<(), Self::Error> {
                Ok(())
            }

            fn export_image(&mut self, _image: ImagePosition) -> Result<(), Self::Error> {
                Ok(())
            }

            fn export_ellipse(&mut self, ellipse: EllipsePosition) -> Result<(), Self::Error> {
                let expected_position = Point2::new(-25. * FRAC_1_SQRT_2, 25. * FRAC_1_SQRT_2);
                assert!(
                    (ellipse.center - expected_position).magnitude() < 10e-6,
                    "left = {}, right = {}",
                    ellipse.center,
                    expected_position,
                );

                Ok(())
            }

            fn export_curve(
                &mut self,
                _curve: CurvePosition,
                StylePosition { fill, stroke }: StylePosition,
            ) -> Result<(), Self::Error> {
                Ok(())
            }

            fn export_text(&mut self, text: TextPosition) -> Result<(), Self::Error> {
                match text.text {
                    "1" => {
                        let expected_position =
                            Point2::new(-25. * FRAC_1_SQRT_2, 25. * FRAC_1_SQRT_2);
                        assert!(
                            (text.reference_start - expected_position).magnitude() < 10e-6,
                            "left = {}, right = {}",
                            text.reference_start,
                            expected_position,
                        );
                    }
                    "2" => {
                        let expected_position = Point2::new(0., 0.);
                        assert!(
                            (text.reference_start - expected_position).magnitude() < 10e-6,
                            "left = {}, right = {}",
                            text.reference_start,
                            expected_position,
                        );
                    }
                    "3" => {
                        let expected_position =
                            Point2::new(30. * FRAC_1_SQRT_2, -30. * FRAC_1_SQRT_2);
                        assert!(
                            (text.reference_start - expected_position).magnitude() < 10e-6,
                            "left = {}, right = {}",
                            text.reference_start,
                            expected_position,
                        );
                    }
                    _ => {}
                }

                Ok(())
            }
        }

        dessin
            .write_into_exporter(
                &mut Exp,
                &Default::default(),
                StylePosition {
                    fill: None,
                    stroke: None,
                },
            )
            .unwrap();
    }
}
