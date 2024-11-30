use num::Complex;
use rand::Rng;
use std::env;
use std::fmt::Display;
use std::str::FromStr;

use rayon::prelude::*;

static MAX: u16 = 14000; //65535;
static MAX_U16: u16 = 65535;
static RED_JUMP: u16 = 100;
static GREEN_JUMP: u16 = 200;
static BLUE_JUMP: u16 = 600;

fn rgb_granient(i: u16) -> (u16, u16, u16) {
    (i * RED_JUMP, i * GREEN_JUMP, i * BLUE_JUMP)
}

fn escape_time(c: Complex<f64>, limit: u16) -> Option<(u16, u16, u16)> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(rgb_granient(i));
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}
#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

fn render(
    pixels: &mut [(u16, u16, u16)],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, MAX) {
                None => (0, 0, 0),
                Some(count) => (MAX_U16 - count.0, MAX_U16 - count.1, MAX_U16 - count.2),
            }
        }
    }
}

//Handling multiple type of errors in Result (to be able to use '?')
#[derive(Debug)]
enum WriteImageError {
    ImageError(image::error::ImageError),
    IoError(std::io::Error),
}

impl Display for WriteImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteImageError::ImageError(image_error) => write!(f, "{}", image_error),
            WriteImageError::IoError(io_error) => write!(f, "{}", io_error),
        }
    }
}

impl std::error::Error for WriteImageError {}

impl From<image::error::ImageError> for WriteImageError {
    fn from(err: image::error::ImageError) -> Self {
        WriteImageError::ImageError(err)
    }
}

impl From<std::io::Error> for WriteImageError {
    fn from(err: std::io::Error) -> Self {
        WriteImageError::IoError(err)
    }
}

fn write_image(
    file_name: &str,
    pixels: &[(u16, u16, u16)],
    bounds: (usize, usize),
) -> Result<(), WriteImageError> {
    let mut imgbuf = image::ImageBuffer::new(bounds.0 as u32, bounds.1 as u32);
    let mut i = 0;
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([pixels[i].0, pixels[i].1, pixels[i].2]);
        i += 1;
    }
    imgbuf.save(file_name).unwrap();

    Ok(())
}

// multithread main
fn multi_thread_old() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} file pixels upper_left, lower_right", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,2.0",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lover right point");

    let mut pixels = vec![(0, 0, 0); bounds.0 * bounds.1];
    let threads = 16;
    let rows_per_band = bounds.1 / threads + 1;

    {
        let bands: Vec<&mut [(u16, u16, u16)]> =
            pixels.chunks_mut(rows_per_band * bounds.0).collect();

        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);

                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn multi_thread(
    file_name: &str,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    let mut pixels = vec![(0, 0, 0); bounds.0 * bounds.1];

    {
        let bands: Vec<(usize, &mut [(u16, u16, u16)])> =
            pixels.chunks_mut(bounds.0).enumerate().collect();
        bands.into_par_iter().for_each(|(i, band)| {
            let top = i;
            let band_bounds = (bounds.0, 1);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        })
    }

    write_image(file_name, &pixels, bounds).expect("error writing PNG file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 && args.len() != 4 {
        eprintln!("Usage: {} file pixels upper_left, lower_right", args[0]);
        eprintln!("Or Usage: {} random count_to_generte pixels", args[0]);
        eprintln!(
            "Example1: {} mandel.png 1000x750 -1.20,0.35 -1.0,2.0",
            args[0]
        );
        eprintln!("Example2: {} random 10 1920x1080", args[0]);
        std::process::exit(1);
    }
    if &args[1] == "random" {
        let count = &args[2];
        let bounds: (usize, usize) =
            parse_pair(&args[3], 'x').expect("Error parsing image dimensions");
        for i in 1..count.parse::<i32>().unwrap() {
            let file_name = format!("madnel{}.png", i);
            let x1: f64 = rand::thread_rng().gen_range(-2.5..2.5);
            let y1: f64 = rand::thread_rng().gen_range(-2.5..2.5);
            let x2: f64 = rand::thread_rng().gen_range(-2.5..2.5);
            let y2: f64 = rand::thread_rng().gen_range(-2.5..2.5);

            println!(
                "x1: {:.20} y1: {:.20} x2: {:.20} xy: {:.20}",
                x1, y1, x2, y2
            );

            let upper_left = parse_complex(&format!("{:.20},{:.20}", x1, y1))
                .expect("Error parsing upper left point");
            let lower_right = parse_complex(&format!("{:.20},{:.20}", x2, y2))
                .expect("Error parsing lover right point");
            multi_thread(&file_name, bounds, upper_left, lower_right)
        }
    } else {
        let file_name = &args[1];
        let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
        let upper_left = parse_complex(&args[3]).expect("Error parsing upper left point");
        let lower_right = parse_complex(&args[4]).expect("Error parsing lover right point");
        multi_thread(file_name, bounds, upper_left, lower_right)
    }
}
