use ::image::{DynamicImage, RgbaImage};
use dessin::{
    export::{Export, Exporter},
    prelude::*,
};
use nalgebra::{Point2, Transform2, Translation2, Vector2};
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, Point, SolidSource, Source,
    StrokeStyle,
};
use std::fmt;

#[derive(Debug)]
pub enum ImageError {
    WriteError(fmt::Error),
    CurveHasNoStartingPoint(CurvePosition),
    FontLoadingError(font_kit::error::FontLoadingError),
    ImageError,
}
impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl From<fmt::Error> for ImageError {
    fn from(value: fmt::Error) -> Self {
        ImageError::WriteError(value)
    }
}
impl std::error::Error for ImageError {}

#[derive(Default)]
pub struct ImageOptions {
    pub canvas: Option<(f32, f32)>,
}

pub struct ImageExporter {
    buffer: DrawTarget,
    style: Vec<StylePosition>,
}

impl ImageExporter {
    fn new(width: u32, height: u32) -> Self {
        ImageExporter {
            buffer: DrawTarget::new(width as i32, height as i32),
            style: vec![],
        }
    }

    fn finalize(self) -> DrawTarget {
        self.buffer
    }

    fn style(&self) -> StylePosition {
        let mut acc = StylePosition {
            stroke: None,
            fill: None,
        };

        for style in self.style.iter().rev() {
            match (acc.fill, style.fill) {
                (None, Some(s)) => acc.fill = Some(s),
                _ => {}
            }

            match (acc.stroke, style.stroke) {
                (None, Some(s)) => acc.stroke = Some(s),
                _ => {}
            }

            if acc.fill.is_some() && acc.fill.is_some() {
                break;
            }
        }

        acc
    }
}

impl Exporter for ImageExporter {
    type Error = ImageError;
    const CAN_EXPORT_ELLIPSE: bool = false;

    fn start_style(&mut self, style: StylePosition) -> Result<(), Self::Error> {
        self.style.push(style);
        Ok(())
    }

    fn end_style(&mut self) -> Result<(), Self::Error> {
        self.style.pop();
        Ok(())
    }

    fn export_image(
        &mut self,
        ImagePosition {
            top_left: _,
            top_right: _,
            bottom_right: _,
            bottom_left: _,
            center: _,
            width: _,
            height: _,
            rotation: _,
            image: _,
        }: ImagePosition,
    ) -> Result<(), Self::Error> {
        // let mut raw_image = Cursor::new(vec![]);
        // image.write_to(&mut raw_image, ImageFormat::Png).unwrap();

        // let data = data_encoding::BASE64.encode(&raw_image.into_inner());

        // write!(
        //     self.acc,
        //     r#"<image width="{width}" height="{height}" x="{x}" y="{y}" "#,
        //     x = center.x - width / 2.,
        //     y = center.y - height / 2.,
        // )?;

        // if rotation.abs() > 10e-6 {
        //     write!(
        //         self.acc,
        //         r#" transform="rotate({rot})" "#,
        //         rot = -rotation.to_degrees()
        //     )?;
        // }

        // write!(self.acc, r#"href="data:image/png;base64,{data}"/>"#,)?;

        Ok(())
    }

    fn export_curve(&mut self, curve: CurvePosition) -> Result<(), Self::Error> {
        let mut path = PathBuilder::new();

        for (idx, k) in curve.keypoints.iter().enumerate() {
            let is_first = idx == 0;

            match k {
                KeypointPosition::Point(p) if is_first => path.move_to(p.x, p.y),
                KeypointPosition::Point(p) => path.line_to(p.x, p.y),
                KeypointPosition::Bezier(b) => {
                    match (is_first, b.start) {
                        (true, None) => return Err(ImageError::CurveHasNoStartingPoint(curve)),
                        (true, Some(s)) => path.move_to(s.x, s.y),
                        (false, None) => {}
                        (false, Some(s)) => path.line_to(s.x, s.y),
                    }

                    path.cubic_to(
                        b.start_control.x,
                        b.start_control.y,
                        b.end_control.x,
                        b.end_control.y,
                        b.end.x,
                        b.end.y,
                    );
                }
            }
        }

        if curve.closed {
            path.close()
        }

        let path = path.finish();

        let style = self.style();

        if let Some(color) = style.fill {
            let (r, g, b, a) = (
                color.into_format::<u8, f32>().red,
                color.into_format::<u8, f32>().green,
                color.into_format::<u8, f32>().blue,
                color.into_format::<u8, u8>().alpha,
            );
            self.buffer.fill(
                &path,
                &Source::Solid(SolidSource { r: b, g, b: r, a }),
                &DrawOptions::new(),
            )
        }

        match style.stroke {
            Some(Stroke::Full { color, width }) => {
                let (r, g, b, a) = (
                    color.into_format::<u8, f32>().red,
                    color.into_format::<u8, f32>().green,
                    color.into_format::<u8, f32>().blue,
                    color.into_format::<u8, u8>().alpha,
                );
                self.buffer.stroke(
                    &path,
                    &Source::Solid(SolidSource { r: b, g, b: r, a }),
                    &StrokeStyle {
                        cap: LineCap::Butt,
                        join: LineJoin::Miter,
                        width,
                        miter_limit: 2.,
                        dash_array: vec![],
                        dash_offset: 0.,
                    },
                    &DrawOptions::new(),
                );
            }
            Some(Stroke::Dashed {
                color,
                width,
                on,
                off,
            }) => {
                let (r, g, b, a) = (
                    color.into_format::<u8, f32>().red,
                    color.into_format::<u8, f32>().green,
                    color.into_format::<u8, f32>().blue,
                    color.into_format::<u8, u8>().alpha,
                );
                self.buffer.stroke(
                    &path,
                    &Source::Solid(SolidSource { r: b, g, b: r, a }),
                    &StrokeStyle {
                        cap: LineCap::Butt,
                        join: LineJoin::Miter,
                        width,
                        miter_limit: 2.,
                        dash_array: vec![on, off],
                        dash_offset: 0.,
                    },
                    &DrawOptions::new(),
                );
            }
            None => {}
        }

        Ok(())
    }

    fn export_text(
        &mut self,
        TextPosition {
            text,
            align: _,
            font_weight,
            on_curve: _,
            font_size,
            reference_start,
            direction: _,
            font,
        }: TextPosition,
    ) -> Result<(), Self::Error> {
        let font = font.clone().unwrap_or_default();
        let fg = dessin::font::get(font);
        let font = fg.get(font_weight).as_bytes();

        //dt.set_transform(&Transform::create_translation(50.0, 0.0));
        // dt.set_transform(&Transform::rotation(euclid::Angle::degrees(15.0)));

        let color = match self.style().fill {
            Some(color) => color,
            None => return Ok(()),
        };
        let (r, g, b, a) = (
            color.into_format::<u8, f32>().red, //-----------------------------------------------------------------------------
            color.into_format::<u8, f32>().green, //before : color.rgba();
            color.into_format::<u8, f32>().blue, //rgba() modification should be better
            color.into_format::<u8, u8>().alpha, //-----------------------------------------------------------------------------
        );

        let font = font_kit::loader::Loader::from_bytes(std::sync::Arc::new(font.to_vec()), 0)
            .map_err(|e| ImageError::FontLoadingError(e))?;
        self.buffer.draw_text(
            &font,
            font_size,
            text,
            Point::new(reference_start.x, reference_start.y),
            &Source::Solid(SolidSource { r: b, g, b: r, a }),
            &DrawOptions::new(),
        );

        Ok(())
    }
}

pub trait ToImage {
    fn rasterize(&self) -> Result<DynamicImage, ImageError>;
}

impl ToImage for Shape {
    fn rasterize(&self) -> Result<DynamicImage, ImageError> {
        let bb = self.local_bounding_box().straigthen();

        let center: Vector2<f32> = bb.center() - Point2::origin();
        let translation =
            Translation2::from(Vector2::new(bb.width() / 2., bb.height() / 2.) - center);
        let scale = nalgebra::Scale2::new(1., -1.);
        let transform = nalgebra::convert::<_, Transform2<f32>>(translation)
            * nalgebra::convert::<_, Transform2<f32>>(scale);

        let width = bb.width().ceil() as u32;
        let height = bb.height().ceil() as u32;
        let mut exporter = ImageExporter::new(width, height);

        self.write_into_exporter(&mut exporter, &transform)?;

        let raw: Vec<u32> = exporter.finalize().into_vec();
        let raw: Vec<u8> = unsafe {
            let cap = raw.capacity();
            let len = raw.len();
            let ptr = Box::into_raw(raw.into_boxed_slice());

            Vec::from_raw_parts(ptr.cast(), len * 4, cap * 4)
        };

        let img = DynamicImage::ImageRgba8(
            RgbaImage::from_raw(width, height, raw).ok_or(ImageError::ImageError)?,
        );

        Ok(img)
    }
}
