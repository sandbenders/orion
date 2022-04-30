#[allow(dead_code, unused)]         // remove that later

use rand::Rng;

#[derive(Debug)]
struct Points {
    pos_x: u16,                     // unsigned (0/+) max 65535
    pos_y: u16,
}

impl Points {
    fn generate_points(&self) {
        let mut rng = rand::thread_rng();
        let number_points = rng.gen_range(5..10);
        println!("{}", number_points);
    }
}

fn main() {
    let mut vec_points: Vec<Points> = Vec::new();
    let point1 = Points {
        pos_x: 10,
        pos_y: 20,
    };

    point1.generate_points();

    vec_points.push(point1);

    println!("{:?}", vec_points);
}
