# [dessin](https://docs.rs/dessin/)

**dessin is library aimed at building complex drawings, combine them, move them and export them as PDF or SVG.**

## Getting started

Add `dessin` and `dessin-svg` to your project dependencies

```bash
cargo add dessin dessin-svg
```

or if you need PDF:

```bash
cargo add dessin dessin-pdf
```

Documentation on [docs.rs](https://docs.rs/dessin/0.8.2-pre/)

### Overview

```rust
use dessin::prelude::*;
use palette::{named, Srgb};

#[derive(Default, Shape)]
struct MyShape {
  text: String,
}
impl MyShape {
  fn say_this(&mut self, what: &str) {
    self.text = format!("{} And check this out: `{what}`", self.text);
  }
}
impl From<MyShape> for Shape {
  fn from(MyShape { text }: MyShape) -> Self {
    dessin2!(Text!(fill = Srgb::<f32>::from_format(named::RED).into_linear(), { text })).into()
  }
}

fn main() {
  let dessin = dessin2!(for x in 0..10 {
    let radius = x as f32 * 10.;

    dessin2!([
      Circle!(
        fill = Srgb::<f32>::from_format(named::RED).into_linear(),
        { radius },
        translate = [x as f32 * 5., 10.],
      ),
      Text!(fill = Srgb::<f32>::from_format(named::BLACK).into_linear(), font_size = 10., text = "Hi !",),
    ])
  });

  let dessin = dessin2!([
    { dessin }(scale = [2., 2.]),
    MyShape(say_this = "Hello world"),
  ]);

  let svg = dessin_svg::to_string(&dessin).unwrap();
}

```
