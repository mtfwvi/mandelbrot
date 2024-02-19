use std::io::{stdin, stdout, Write};
use image::RgbImage;
use num::Complex;
use clap::Parser;
use spinners::{Spinner, Spinners};
use std::path::Path;

const XMIN: f64 = -2.2f64;
const XMAX: f64 = 1.2f64;
const YMIN: f64 = -2.0f64;
const YMAX: f64 = 2.0f64;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];
const ALLOWED_EXTENSIONS: [&str; 6] = [
    "PNG", "JPG", "JPEG", "TIFF", "WEBP", "GIF"
];

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

    /* Get all arguments from clap */
    let args = Args::parse();

    let width = args.width;
    let height = args.height;
    let file = args.filename;
    let invert = args.invert;
    let dynamic = args.dynamic;
    let maxiterations = args.iterations;
    let limit = args.limit;

    /*
    Check if the file extension is valid
                                    I hate this.
                                     ↓        ↓  */
    if !ALLOWED_EXTENSIONS.contains(&&**&Path::new(&file).extension().unwrap().to_str().unwrap().to_uppercase()) {
        println!("ERROR: {:?} is not a valid file extension! Aborting.", Path::new(&file).extension().unwrap());
        return;
    }

    /* Check if the file already exists, and prompt the user about how to continue */
    if Path::new(&file).is_file() {
        let mut input = String::new();
        print!("File {} already exists. Override? (Y/N): ", file);
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Please enter a valid string.");

        if input.trim().to_uppercase().to_string() == "N" || input.trim().to_uppercase().to_string() == "NO" {
            return;
        } else if input.trim().to_uppercase().to_string() == "Y" || input.trim().to_uppercase().to_string() == "YES" {
            // Continue. Somehow a negative elseif is not working...
        } else {
            println!("ERROR: Please enter Y(es) or (N)o. Aborting.");
            return;
        }
    }

    /* Warn the user if the file dimensions are too large */
    if width > 4000 || height > 4000 {
        println!("WARN: Your image size ({width}x{height}) seems to be very large.");
        println!("      Be aware: This can take a long time to generate and might create");
        println!("      memory leaks. You have been warned!");
    }

    /* Create a spinner to display progress */
    let mut spinner = Spinner::new(Spinners::Line, "Generating image...".into());

    /* Create an image buffer */
    let mut image = RgbImage::new(width as u32, height as u32);

    let col_div: f64 = maxiterations as f64 / 255.0;

    /* Iterate through all pixels (vertical) */
    for x in 0..height {
        let re: f64 = XMIN + (x as f64 / (width as f64 - 1.0)) * (XMAX - XMIN);
        /* Iterate through all pixels (horizontal) */
        for y in 0..width {
            let im: f64 = YMIN + (y as f64 / (height as f64 - 1.0)) * (YMAX - YMIN);
            let c: Complex<f64> = Complex::new(re, im);
            let mb: i64 = mandelbrot(c, maxiterations, limit);

            /* Check color for pixel */
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

            /* Set pixel in image buffer */
            *image.get_pixel_mut(x as u32, y as u32) = image::Rgb(col);
        }
    }

    /*
    Save the file
    Be sure to keep this line above spinner.stop_and_persist(), as saving large files could take some time
    */
    image.save(file.clone()).unwrap();

    /* Stop the spinner */
    spinner.stop_and_persist("\x1b[32m🗸\x1b[0m", "Done! File saved under ".to_owned() + &file);

    return;
}