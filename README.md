## Mandelbrot in Rust

![WTFPL](http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-1.png)

Generate a mandelbrot set image with rust.

#### Usage

```
Usage: mandelbrot [OPTIONS]

Options:
  -f, --filename <FILENAME>  Name of your generated image [default: mandelbrot.png]
      --width <WIDTH>        Width of your output image [default: 500]
      --height <HEIGHT>      Height of your output image [default: 500]
  -d, --dynamic              Enable color gradients and shades
  -i, --invert               Invert your image colors
  -h, --help                 Print help
  -V, --version              Print version

```

#### Example

![Mandelbrot Demo image](demo/mandelbrot.png)

#### Customizing

Most values can be customized by using the CLI (see [Usage](#usage)). Some values that can't be tweaked (yet) via the CLI can be changed by modifying the following constants (found at the beginning of `src/main.rs`):

```rs
const MAXITERATIONS: i32 = 255; // Maximum amount of iterations
const LIMIT: f32 = 5.0;

// Complex plane boundaries
const XMIN: f32 = -2.2 as f32;
const XMAX: f32 = 1.2 as f32;
const YMIN: f32 = -2.0 as f32;
const YMAX: f32 = 2.0 as f32;

// Color definitions
const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];
```

#### Learn more

[More information about the mandelbrot set](https://www.math.utah.edu/~alfeld/math/mandelbrot/mandelbrot.html) (University of Utah)