use clap;
use num::complex::Complex;
use raster;


struct Config {
    width: i32,
    height: i32,
    max_depth: u64,
    re_start: f64,
    re_end: f64,
    im_start: f64,
    im_end: f64,
}


fn main() {
    let config = parse_args();
    let mut image = raster::Image::blank(config.width, config.height);

    for x in 0..config.width {
        for y in 0..config.height {
            let m = mandelbrot(
                config.re_start + (x as f64 / config.width as f64) * (config.re_end - config.re_start),
                config.im_start + (y as f64 / config.height as f64) * (config.im_end - config.im_start),
                config.max_depth,
            );

            image.set_pixel(x, y, map_to_color(m, &config)).unwrap();
        }
    }
    raster::save(&image, "/tmp/mandelbrot.png").unwrap();
}


fn parse_args() -> Config {
    let matches = clap::App::new("Mandelbrot image generator")
        .arg(clap::Arg::with_name("width").short("w").long("width").takes_value(true))
        .arg(clap::Arg::with_name("height").short("h").long("height").takes_value(true))
        .arg(clap::Arg::with_name("max_depth").short("d").long("max-depth").takes_value(true))        
        .arg(clap::Arg::with_name("re_start").long("re-start").takes_value(true))
        .arg(clap::Arg::with_name("re_end").long("re-end").takes_value(true))
        .arg(clap::Arg::with_name("im_start").long("im-start").takes_value(true))
        .arg(clap::Arg::with_name("im_end").long("im-end").takes_value(true))
        .get_matches();

    Config {
        width: matches.value_of("width").unwrap_or("800").parse().unwrap(),
        height: matches.value_of("height").unwrap_or("600").parse().unwrap(),
        max_depth: matches.value_of("max_depth").unwrap_or("100").parse().unwrap(),
        re_start: matches.value_of("re_start").unwrap_or("-2.0").parse().unwrap(),
        re_end: matches.value_of("re_start").unwrap_or("1.0").parse().unwrap(),
        im_start: matches.value_of("re_start").unwrap_or("-1.0").parse().unwrap(),
        im_end: matches.value_of("re_start").unwrap_or("1.0").parse().unwrap(),
    }
}

fn mandelbrot(x: f64, y: f64, max_depth: u64) -> f64 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);
    let mut n_iter = 0;

    while n_iter < max_depth && z.norm() <= 2.0 {
        z = z * z + c;
        n_iter += 1;
    }

    n_iter as f64 - z.norm().ln().ln() / 2.0f64.ln()
}


const MAX_HSV_HUE:u16 = 360;
const MAX_HSV_SATURATION:f32 = 100.0;
const MAX_HSV_VALUE:f32 = 100.0;


fn map_to_color(mandelbrot: f64, config: &Config) -> raster::Color {
    let rgb = raster::Color::to_rgb(
        (MAX_HSV_HUE as f64 * mandelbrot as f64 / config.max_depth as f64) as u16,
        MAX_HSV_SATURATION,
        if mandelbrot < config.max_depth as f64 { MAX_HSV_VALUE } else { 0.0 },
    );

    raster::Color::rgb(rgb.0, rgb.1, rgb.2)
}
