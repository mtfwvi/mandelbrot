## Mandelbrot in Rust

CLI Tool to generate mandelbrot set plots using Rust.

### Usage

```
CLI Tool to generate mandelbrot set plots using Rust.

Usage: mandelbrot [OPTIONS]

Options:
  -f, --filename <FILENAME>      Name of your generated image [default: mandelbrot.png]
      --width <WIDTH>            Width of your output image [default: 500]
      --height <HEIGHT>          Height of your output image [default: 500]
  -d, --dynamic                  Enable color gradients and shades
  -i, --invert                   Invert your image colors
      --iterations <ITERATIONS>  Maximum amount of iterations [default: 255]
      --limit <LIMIT>            Limit for the absolute value of the complex number [default: 5]
  -h, --help                     Print help
  -V, --version                  Print version
```

### Supported formats

Among the supported formats are:
- PNG
- JPG
- WEBP
- TIFF

### Example

![Mandelbrot Demo image](demo/mandelbrot.png)

### Customizing

Most values can be customized by using the CLI (see [Usage](#usage)). Some values that can't be tweaked (yet) via the CLI can be changed by modifying the following constants (found at the beginning of `src/main.rs`):

```rs
// Complex plane boundaries
const XMIN: f64 = -2.2 as f64;
const XMAX: f64 = 1.2 as f64;
const YMIN: f64 = -2.0 as f64;
const YMAX: f64 = 2.0 as f64;

// Color definitions
const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];
```

### Learn more

[More information about the mandelbrot set](https://www.math.utah.edu/~alfeld/math/mandelbrot/mandelbrot.html) (University of Utah)