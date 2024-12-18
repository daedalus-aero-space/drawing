use crate::prelude::*;
use nalgebra::Transform2;
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

pub type Shaper = dyn Fn() -> Shape;

pub trait DynamicShape: std::fmt::Debug {
    fn as_shape(&self) -> Shape;
}

impl<T: std::fmt::Debug + Clone + Into<Shape>> DynamicShape for T {
    fn as_shape(&self) -> Shape {
        self.clone().into()
    }
}

#[derive(Clone, Debug, Shape)]
pub struct Dynamic<T> {
    #[local_transform]
    local_transform: Transform2<f32>,
    #[shape(skip)]
    shape: T,
    #[shape(skip)]
    _ref: Option<Arc<RwLock<T>>>,
}

impl<T: Default> Default for Dynamic<T> {
    fn default() -> Self {
        Dynamic {
            shape: T::default(),
            local_transform: Default::default(),
            _ref: Default::default(),
        }
    }
}

impl<T> Deref for Dynamic<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<T> DerefMut for Dynamic<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl<T> From<Dynamic<T>> for Shape
where
    T: DynamicShape + 'static,
{
    fn from(
        Dynamic {
            local_transform,
            shape,
            _ref,
        }: Dynamic<T>,
    ) -> Self {
        let _ref = match _ref {
            Some(_ref) => {
                *_ref.write().unwrap() = shape;
                _ref
            }
            None => Arc::new(RwLock::new(shape)),
        };

        Shape::Dynamic {
            local_transform,
            shaper: Arc::new(move || _ref.read().unwrap().as_shape()),
        }
    }
}

impl<T> Dynamic<T> {
    pub fn _ref(&mut self, _ref: &Arc<RwLock<T>>) -> &mut Self {
        self._ref = Some(_ref.clone());
        self
    }

    pub fn with_ref(mut self, _ref: &Arc<RwLock<T>>) -> Self {
        self._ref(_ref);
        self
    }
}

#[test]
fn dynamic() {
    use crate::export::{Export, Exporter};
    use nalgebra::Point2;

    struct TestExporter(f32);
    impl Exporter for TestExporter {
        type Error = ();

        fn start_style(&mut self, _style: StylePosition) -> Result<(), Self::Error> {
            unimplemented!()
        }

        fn end_style(&mut self) -> Result<(), Self::Error> {
            unimplemented!()
        }

        fn export_image(&mut self, _image: ImagePosition) -> Result<(), Self::Error> {
            unimplemented!()
        }

        fn export_ellipse(&mut self, ellipse: EllipsePosition) -> Result<(), Self::Error> {
            assert_eq!(ellipse.center, Point2::new(0., 0.));
            assert_eq!(ellipse.semi_major_axis, self.0);
            assert_eq!(ellipse.semi_minor_axis, self.0);
            Ok(())
        }

        fn export_curve(
            &mut self,
            _curve: CurvePosition,
            StylePosition { fill, stroke }: StylePosition,
        ) -> Result<(), Self::Error> {
            unimplemented!()
        }

        fn export_text(&mut self, _text: TextPosition) -> Result<(), Self::Error> {
            unimplemented!()
        }
    }

    let my_ref = Default::default();

    let c = dessin2!([Dynamic::<Circle>(_ref = &my_ref, radius = 2.,)]);
    c.write_into_exporter(
        &mut TestExporter(2.),
        &Transform2::default(),
        StylePosition {
            fill: None,
            stroke: None,
        },
    )
    .unwrap();

    my_ref.write().unwrap().scale([10., 10.]);
    c.write_into_exporter(
        &mut TestExporter(20.),
        &Transform2::default(),
        StylePosition {
            fill: None,
            stroke: None,
        },
    )
    .unwrap();
}
