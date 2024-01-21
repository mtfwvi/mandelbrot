use image::RgbImage;
use num::Complex;
use clap::Parser;
use spinners::{Spinner, Spinners};

const XMIN: f64 = -2.2 as f64;
const XMAX: f64 = 1.2 as f64;
const YMIN: f64 = -2.0 as f64;
const YMAX: f64 = 2.0 as f64;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of your generated image
    #[arg(short, long, default_value_t = String::from("mandelbrot.png"))]
    filename: String,

    /// Width of your output image
    #[arg(long, default_value_t = 500)]
    width: i64,

    /// Height of your output image
    #[arg(long, default_value_t = 500)]
    height: i64,

    /// Enable color gradients and shades
    #[arg(short, long, default_value_t = false)]
    dynamic: bool,

    /// Invert your image colors
    #[arg(short, long, default_value_t = false)]
    invert: bool,

    /// Maximum amount of iterations
    #[arg(long, default_value_t = 255)]
    iterations: i64,

    /// Limit for the absolute value of the complex number
    #[arg(long, default_value_t = 5)]
    limit: i64,
}

fn mandelbrot(c: Complex<f64>, m: i64, l: i64) -> i64 {
    let mut z = c;

    for i in 0..m {
        if z.norm() > l as f64 {
            return i;
        }
        z = z * z + c;
    }

    return m;
}

fn main() {

    let args = Args::parse();

    let width = args.width;
    let height = args.height;
    let file = args.filename;
    let invert = args.invert;
    let dynamic = args.dynamic;
    let maxiterations = args.iterations;
    let limit = args.limit;

    let mut sp = Spinner::new(Spinners::Line, "Generating image...".into());

    let mut image = RgbImage::new(width as u32, height as u32);

    let col_div: f64 = maxiterations as f64 / 255.0;

    for x in 0..height {
        let re: f64 = XMIN + (x as f64 / (width as f64 - 1.0)) * (XMAX - XMIN);

        for y in 0..width {
            let im: f64 = YMIN + (y as f64 / (height as f64 - 1.0)) * (YMAX - YMIN);
            let c: Complex<f64> = Complex::new(re, im);
            let mb: i64 = mandelbrot(c, maxiterations, limit);

            let mut col: [u8; 3];

            if mb < maxiterations {
                if invert { col = WHITE; }
                else { col = BLACK; }
            } else {
                if invert { col = BLACK; }
                else { col = WHITE; }
            }

            if dynamic {
                let shade: f64;
                if invert { shade = 255.0 - (mb as f64 / col_div); }
                else { shade = mb as f64 / col_div; }
                col = [shade as u8, shade as u8, shade as u8];
            }

            *image.get_pixel_mut(x as u32, y as u32) = image::Rgb(col);
        }
    }

    sp.stop_and_persist("\x1b[32m🗸\x1b[0m", "Done! File saved under ".to_owned() + &file);
    image.save(file).unwrap();
}