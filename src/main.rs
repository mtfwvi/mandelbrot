use image::RgbImage;
use num::Complex;
use clap::Parser;

const MAXITERATIONS: i32 = 255;
const LIMIT: f32 = 5.0;

const XMIN: f32 = -2.2 as f32;
const XMAX: f32 = 1.2 as f32;
const YMIN: f32 = -2.0 as f32;
const YMAX: f32 = 2.0 as f32;

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
}

fn main() {

    let args = Args::parse();

    let width = args.width;
    let height = args.height;
    let file = args.filename;
    let invert = args.invert;
    let dynamic = args.dynamic;


    let mut image = RgbImage::new(width as u32, height as u32);

    for x in 0..height {
        let re: f32 = XMIN + (x as f32 / (width as f32 - 1.0)) * (XMAX - XMIN);

        for y in 0..width {
            let im: f32 = YMIN + (y as f32 / (height as f32 - 1.0)) * (YMAX - YMIN);
            let c: Complex<f32> = Complex::new(re, im);
            let mb: i32 = mandelbrot(c);

            if !dynamic {
                if invert {
                    if mb < MAXITERATIONS {
                        *image.get_pixel_mut(x as u32, y as u32) = image::Rgb(WHITE);
                    } else {
                        *image.get_pixel_mut(x as u32, y as u32) = image::Rgb(BLACK);
                    }
                } else {
                    if mb < MAXITERATIONS {
                        *image.get_pixel_mut(x as u32, y as u32) = image::Rgb(BLACK);
                    } else {
                        *image.get_pixel_mut(x as u32, y as u32) = image::Rgb(WHITE);
                    }
                }
            } else {
                if invert {
                    *image.get_pixel_mut(x as u32, y as u32) =
                        image::Rgb([255 - mb as u8, 255 - mb as u8, 255 - mb as u8]);
                } else {
                    *image.get_pixel_mut(x as u32, y as u32) =
                        image::Rgb([mb as u8, mb as u8, mb as u8]);
                }
            }
        }
    }

    image.save(file).unwrap();
}

fn mandelbrot(c: Complex<f32>) -> i32 {
    let mut z = c;

    for i in 0..MAXITERATIONS {
        if z.norm() > LIMIT {
            return i;
        }
        z = z * z + c;
    }

    return MAXITERATIONS;
}
