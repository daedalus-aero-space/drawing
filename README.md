# [dessin](https://docs.rs/dessin/)

**Try out the new [API](https://github.com/432-Technologies/dessin/tree/v0.8-pre)**

Generate complex drawing for PDF, SVG, and many more to come ! 

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
    dessin2!(Text!(fill = Color::RED, { text })).into()
  }
}

fn main() {
  let dessin = dessin2!(for x in 0..10 {
    let radius = x as f32 * 10.;

    dessin2!([
      Circle!(
        fill = Color::RED,
        { radius },
        translate = [x as f32 * 5., 10.],
      ),
      Text!(fill = Color::BLACK, font_size = 10., text = "Hi !",),
    ])
  });

  let dessin = dessin2!([
    { dessin }(scale = [2., 2.]),
    MyShape(say_this = "Hello world"),
  ]);

  let svg = dessin_svg::to_string(&dessin).unwrap();
}

```
