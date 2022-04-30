#[allow(dead_code, unused)] // remove that later
extern crate cairo;

use rand::Rng;

use cairo::{ Context, Format, ImageSurface };

use std::fs::File;

#[derive(Debug)]
struct Points {
    pos_x: u16, // unsigned (0/+) max 65535
    pos_y: u16,
}

fn generate_points() -> Vec<Points> {
    let mut rng = rand::thread_rng();
    let number_points = rng.gen_range(5..10);

    let mut vec_points: Vec<Points> = Vec::new();

    let mut x = 0;
    while x < number_points {
        x += 1;
        let new_point = Points {
            pos_x: rng.gen_range(0..297),
            pos_y: rng.gen_range(0..420),
        };
        vec_points.push(new_point);
    }

    vec_points
}

fn draw_lines(points: &Vec<Points>) {
    dbg!(&points);
    let surface = ImageSurface::create(Format::ARgb32, 297, 420)
        .expect("Couldn't create a surface!");
    let context = Context::new(&surface);

    let mut rng = rand::thread_rng();
    context.set_source_rgb(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
    context.paint();

    let mut file = File::create("output.png")
        .expect("Couldn't create file");
    surface.write_to_png(&mut file)
        .expect("Couldn't write to png");
}

fn main() {
    let points = generate_points();
    draw_lines(&points);
}
