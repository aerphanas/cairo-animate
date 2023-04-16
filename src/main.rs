use cairo::{Context, Format, ImageSurface};
use std::{f64::consts::PI, fs};

fn main() {
    fs::create_dir_all("output").unwrap();

    let surface = ImageSurface::create(Format::ARgb32, 600, 400).unwrap();
    let cr = Context::new(&surface).unwrap();

    let mut angle: f64 = 0.0;
    let mut radius: f64 = 100.0;

    for i in 0..100 {
        cr.set_source_rgba(1.0, 1.0, 1.0, 1.0);
        cr.paint().unwrap();

        cr.arc(300.0, 200.0, radius, 0.0, 2.0 * PI);
        cr.set_source_rgba(0.0, 0.0, 1.0, 1.0);
        cr.set_line_width(3.0);
        cr.stroke().unwrap();

        let x = 300.0 + radius * angle.cos();
        let y = 200.0 + radius * angle.sin();
        cr.arc(x, y, 10.0, 0.0, 2.0 * PI);
        cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
        cr.fill().unwrap();

        angle += 0.1;
        radius += (i as f64) / 100.0;
        let file_name = format!("output/frame{}.png", i);
        let mut file = std::fs::File::create(&file_name).unwrap();
        surface.write_to_png(&mut file).unwrap();
    }
}
