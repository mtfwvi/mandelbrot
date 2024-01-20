use image::RgbImage;
use num::Complex;

const WIDTH: i32 = 750;
const HEIGHT: i32 = WIDTH;
const FILE: &str = "mandelbrot.png";

const MAXITERATIONS: i32 = 255;
const LIMIT: f32 = 5.0;

const XMIN: f32 = -2.2 as f32;
const XMAX: f32 = 1.2 as f32;
const YMIN: f32 = -2.0 as f32;
const YMAX: f32 = 2.0 as f32;

const DYNAMIC: bool = true;
const INVERTED: bool = false;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];

fn main() {
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for x in 0..HEIGHT {
        let re: f32 = XMIN + (x as f32 / (WIDTH as f32 - 1.0)) * (XMAX - XMIN);

        for y in 0..WIDTH {
            let im: f32 = YMIN + (y as f32 / (HEIGHT as f32 - 1.0)) * (YMAX - YMIN);
            let c: Complex<f32> = Complex::new(re, im);
            let mb: i32 = mandelbrot(c);

            if !DYNAMIC {
                if INVERTED {
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
                if INVERTED {
                    *image.get_pixel_mut(x as u32, y as u32) =
                        image::Rgb([255 - mb as u8, 255 - mb as u8, 255 - mb as u8]);
                } else {
                    *image.get_pixel_mut(x as u32, y as u32) =
                        image::Rgb([mb as u8, mb as u8, mb as u8]);
                }
            }
        }
    }

    image.save(FILE).unwrap();
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
