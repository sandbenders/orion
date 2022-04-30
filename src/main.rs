// a3 size in pixels at 300 DPI: 3508 x 4960
#[allow(dead_code, unused)] // remove that later
extern crate cairo;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use cairo::{ Context, Format, ImageSurface };

use std::fs::File;

#[derive(Debug)]
struct Points {
    pos_x: f64,
    pos_y: f64,
}

fn generate_points() -> Vec<Points> {
    let mut rng = thread_rng();
    let number_points = rng.gen_range(5..15);

    let mut vec_points: Vec<Points> = Vec::new();

    let mut x = 0;
    while x < number_points {
        x += 1;
        let new_point = Points {
            pos_x: rng.gen_range(0.0..3508.0),
            pos_y: rng.gen_range(0.0..4960.0),
        };
        vec_points.push(new_point);
    }

    vec_points
}

fn draw_lines(points: &Vec<Points>) {
    let surface = ImageSurface::create(Format::ARgb32, 3508, 4960)
        .expect("Couldn't create a surface!");
    let context = Context::new(&surface);

    let mut rng = thread_rng();
    context.set_source_rgb(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
    context.paint();

    for point in points.iter() {
        context.move_to(point.pos_x, point.pos_y);
        for new_points in points.iter() {
            context.line_to(new_points.pos_x, new_points.pos_y);
        }
        context.close_path();

        context.set_source_rgba(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
        context.fill_preserve();
    }

    let mut filename: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();

    filename.push_str(".png");

    dbg!(&filename);

    let mut file = File::create(&filename)
        .expect("Couldn't create file");
    surface.write_to_png(&mut file)
        .expect("Couldn't write to png");
}

fn main() {
    // generate 10 images at time
    for _ in 0..10 {
        let points = generate_points();
        draw_lines(&points);
    }
}
