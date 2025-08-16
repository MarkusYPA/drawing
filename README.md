# Drawing in Rust

This project is a Rust implementation of a simple shape-drawing system that generates a png image containing a variety of colorful shapes — rectangles, triangles, circles, lines, and points — randomly placed or defined in code.

The shapes are drawn using thin colored lines creating an abstract overlapping effect.

---

## Project Structure

* **`main.rs`** – Entry point of the program.
  Demonstrates usage of the shape structures and traits by drawing several shapes and saving the result as `image.png`.

* **`geometrical_shapes.rs`** – Module containing all the shape definitions, traits, and logic for:

  * `Point`
  * `Line`
  * `Triangle`
  * `Rectangle`
  * `Circle`

---

## Features

* **Traits**

  * `Drawable` – Defines methods for drawing and coloring shapes.
  * `Displayable` – Defines a method for placing colored pixels on an image.

* **Shapes**

  * `Point::new(x, y)`
  * `Line::new(&point_a, &point_b)`
  * `Triangle::new(&p1, &p2, &p3)`
  * `Rectangle::new(&corner1, &corner2)`
  * `Circle::new(&center, radius)`

* **Random Generators**
  The all shapes include a `random(max x, max y)` associated function.

* **Image Output**
  The final image is saved as `image.png` in the project root.

---

## Dependencies

This project uses the [`raster`](https://crates.io/crates/raster) crate for image creation and manipulation and the [`rand`](https://crates.io/crates/rand) crate for randomization.

Add to your `Cargo.toml`:

```toml
[dependencies]
raster = "0.2"
rand = "0.9.2"
```

---

## Running the Project

1. **Clone the repository**

   ```bash
   git clone https://01.gritlab.ax/git/mamberla/drawing.git
   cd drawing
   ```

2. **Build and run**

   ```bash
   cargo run
   ```

3. **View the output**

   * Check the generated file:

     ```
     image.png
     ```

---

## Example Output

The generated image will contain:

* A black background.
* Many overlapping shapes in random colors.
* Lines, rectangles, triangles, circles, and points.

## Authors

- [Roope Hongisto](https://github.com/RuBoMa)
- [Markus Amberla](https://github.com/MarkusYPA)
