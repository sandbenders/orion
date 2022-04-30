#[allow(dead_code, unused)]         // remove that later
#[derive(Debug)]
struct Points {
    pos_x: u16,                     // unsigned (0/+) max 65535
    pos_y: u16,
}

fn main() {
    let mut vec_points: Vec<Points> = Vec::new();
    let point1 = Points {
        pos_x: 10,
        pos_y: 20,
    };

    vec_points.push(point1);

    println!("{:?}", vec_points);
}
