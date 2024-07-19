use dessin::{
    nalgebra::{Point2, Rotation2, Translation2},
    prelude::*,
};
use dessin_image::ToImage;
use dessin_pdf::ToPDF;
use palette::{named, Srgb, Srgba};
use project_root::get_project_root;
use std::f32::consts::{FRAC_PI_4, FRAC_PI_8, PI};

const C: Srgb = Srgb::new(0.231, 0.329, 0.522);
fn c(a: f32) -> Srgba {
    Srgba::new(0.231, 0.329, 0.522, a)
}

#[derive(Default)]
pub struct InnerBubbleRing;
impl From<InnerBubbleRing> for Shape {
    fn from(_: InnerBubbleRing) -> Self {
        let ring_strip = dessin2!(
            [
                Circle!(stroke = Stroke::new_full(c(0.784), 0.1), radius = 1.,),
                Circle!(
                    stroke = Stroke::new_full(c(0.588), 0.1),
                    radius = 0.5,
                    translate = Translation2::new(2., 0.),
                ),
                Circle!(
                    stroke = Stroke::new_full(c(0.392), 0.1),
                    radius = 0.25,
                    translate = Translation2::new(3.2, 0.),
                ),
            ] > (translate = Translation2::new(14., 0.))
        );

        let angle = PI / 14_f32;
        dessin2!(for n in 0..28 {
            dessin2!({ ring_strip.clone() }(
                rotate = Rotation2::new(n as f32 * angle)
            ))
        })
    }
}

#[derive(Default)]
pub struct BinaryRing(pub f32);
impl BinaryRing {
    #[inline]
    pub fn radius(&mut self, radius: f32) -> &mut Self {
        self.0 = radius;
        self
    }
}
impl From<BinaryRing> for Shape {
    fn from(BinaryRing(radius): BinaryRing) -> Self {
        const T: &str = "10001011101001011000101110001010010101110100111010010101110010101001110010100101011010100101111101001011011100001110001110001011100000101011100101000101110100101100010111000101001010111010011101001010101100010111000101001010111010011101001010111001010100111001010010101101010010111110100101101110000111000111000101110000010101110010100010111010010110001011100010100101011101001110100101011100101010011100101001010110101001011111010010110111000011100011100010111000001010111001010001011101001011000101110001010010101110100111010010101110010101001110010100101011010100101111101001011011100001110001110001011100000101011100101000101110100101100010111000101001010111010011101001010111001010100111001010010101101010010111110100101101110000111000111000101110000010101110010";
        dessin2!(
            Text!(
                text = T,
                on_curve = Circle::default().with_radius(radius),
                font_size = 1.,
                fill = C,
            ) > ()
        )
    }
}

#[derive(Default)]
pub struct TimerRing;
impl From<TimerRing> for Shape {
    fn from(_: TimerRing) -> Self {
        let long_line = dessin2!(Line(from = Point2::new(36., 0.), to = Point2::new(28., 0.),));
        let short_line = dessin2!(Line(
            from = Point2::new(36., 0.),
            to = Point2::new(32., 0.),
            rotate = Rotation2::new(FRAC_PI_8),
        ));
        let small_line = dessin2!(Line(from = Point2::new(36., 0.), to = Point2::new(35., 0.),));

        dessin2!(
            [
                Circle(radius = 36.),
                for x in 0..8 {
                    dessin2!({ long_line.clone() }(
                        rotate = Rotation2::new(x as f32 * FRAC_PI_4)
                    ))
                },
                for x in 0..8 {
                    dessin2!({ short_line.clone() }(
                        rotate = Rotation2::new(x as f32 * FRAC_PI_4)
                    ))
                },
                for x in 0..160 {
                    dessin2!({ small_line.clone() }(
                        rotate = Rotation2::new(x as f32 * PI / 160.)
                    ))
                },
            ] > !(stroke = Stroke::new_full(C, 0.2))
        )
        .into()
    }
}

#[derive(Default)]
pub struct ThreeColoredRing;
impl From<ThreeColoredRing> for Shape {
    fn from(_: ThreeColoredRing) -> Self {
        dessin2!([
            Circle!(
                stroke = Stroke::new_full(Srgb::new(0.588, 0.588, 0.588), 0.2),
                radius = 40.,
            ),
            Circle!(
                stroke = Stroke::new_full(Srgb::new(0.180, 0.180, 0.180), 0.2),
                radius = 42.,
            ),
            Circle!(stroke = Stroke::new_full(C, 0.2), radius = 44.,),
        ])
    }
}

#[derive(Default)]
pub struct Squares;
impl From<Squares> for Shape {
    fn from(_: Squares) -> Self {
        let square_line = dessin2!(
            [
                Rectangle!(stroke = Stroke::new_full(C, 0.1), width = 2.5, height = 2.5,),
                Rectangle!(
                    stroke = Stroke::new_full(c(0.784), 0.1),
                    width = 1.8,
                    height = 1.8,
                    translate = Translation2::new(2.8, 0.),
                ),
                Rectangle!(
                    stroke = Stroke::new_full(c(0.588), 0.1),
                    width = 1.2,
                    height = 1.2,
                    translate = Translation2::new(4.8, 0.),
                ),
                Rectangle!(
                    stroke = Stroke::new_full(c(0.392), 0.1),
                    width = 0.8,
                    height = 0.8,
                    translate = Translation2::new(6.2, 0.),
                ),
                Rectangle!(
                    stroke = Stroke::new_full(c(0.196), 0.1),
                    width = 0.4,
                    height = 0.4,
                    translate = Translation2::new(7.2, 0.),
                ),
                Rectangle!(
                    stroke = Stroke::new_full(c(0.098), 0.1),
                    width = 0.2,
                    height = 0.2,
                    translate = Translation2::new(7.8, 0.),
                ),
            ] > (translate = Translation2::new(50., 0.))
        );

        let angle = 150_f32.to_radians() / 36.;
        let quarter = dessin2!(for x in 0..36 {
            dessin2!({ square_line.clone() }(
                rotate = Rotation2::new(x as f32 * angle)
            ))
        });

        dessin2!([
            { quarter.clone() }(rotate = Rotation2::new(15_f32.to_radians())),
            { quarter }(rotate = Rotation2::new(195_f32.to_radians())),
        ])
    }
}

#[derive(Default)]
pub struct Symbol432;
impl From<Symbol432> for Shape {
    fn from(_: Symbol432) -> Self {
        dessin2!([
            Curve!(
                stroke = Stroke::new_full(Srgb::new(0.498, 0.498, 0.498), 0.6),
                then = Point2::new(0., 0.),
                then = Point2::new(0., 20.),
                then = Point2::new(-9.8, 0.),
                then = Point2::new(-8., 0.),
            ),
            Line!(
                stroke = Stroke::new_full(Srgb::new(0.0, 0.008, 0.376), 0.6),
                from = [-10., 0.],
                to = [13., 0.],
            ),
            Line!(
                stroke = Stroke::new_full(Srgb::new(0.0, 0.008, 0.376), 0.6),
                from = [0., 0.],
                to = [0., -10.],
            ),
            Text!(
                fill = Srgba::new(0.0, 0.008, 0.376, 0.6),
                text = "echnologies",
                font_size = 2.5,
                font_weight = FontWeight::Bold,
                translate = [0.5, -10.],
                vertical_align = TextVerticalAlign::Center,
                align = TextAlign::Left,
            ),
            Text!(
                fill = Srgb::<f32>::from_format(named::BLACK).into_linear(),
                text = "3",
                font_size = 7.,
                font_weight = FontWeight::Regular,
                translate = [1., 1.],
                vertical_align = TextVerticalAlign::Center,
                align = TextAlign::Left,
            ),
            Text!(
                fill = Srgb::<f32>::from_format(named::BLACK).into_linear(),
                text = "2",
                font_size = 7.,
                font_weight = FontWeight::Regular,
                translate = [1., -5.6],
                vertical_align = TextVerticalAlign::Center,
                align = TextAlign::Left,
            ),
        ])
    }
}

#[derive(Default)]
pub struct Logo432;
impl From<Logo432> for Shape {
    fn from(_: Logo432) -> Self {
        dessin2!([
            InnerBubbleRing(),
            BinaryRing(radius = 10.),
            TimerRing(),
            ThreeColoredRing(),
            Squares(),
            BinaryRing(radius = 30.),
            Circle!(stroke = Stroke::new_full(c(0.588), 0.2), radius = 70.,),
            Symbol432() > (scale = [4., 4.], translate = [-20., -20.],),
        ])
    }
}

fn main() {
    let dessin = Shape::from(Logo432);

    let path = get_project_root().unwrap().join("examples/out/");

    // Image
    dessin2!({ dessin }(scale = [5., 5.]))
        .rasterize()
        .unwrap()
        .into_rgba8()
        .save(path.join("432technologies.png"))
        .unwrap();
}
