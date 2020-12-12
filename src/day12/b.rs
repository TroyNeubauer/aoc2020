use std::io::Read;
use std::collections::HashMap;
use vector2d::Vector2D;



#[test]
fn test() {
let input = String::from(
r#"F10
N3
F7
R90
F11
"#);
    let lines = input.lines().collect();
    assert_eq!(execute(lines), 286);

}

fn execute(lines: Vec<&str>) -> i32 {
    let mut way = Vector2D::new(10_f32, 1_f32);
    let mut pos: Vector2D<f32> = Vector2D::new(0_f32, 0_f32);
    for line in lines {
        println!("ship at {:?} - way {:?}", pos, way);
        let param = &line[1..].parse::<f32>().unwrap();
        let mut rads = param / 180_f32 * std::f32::consts::PI;
        match line.chars().next().unwrap() {
            'N' => way.y += param,
            'S' => way.y -= param,
            'E' => way.x += param,
            'W' => way.x -= param,
            'L' => way = Vector2D::new(way.x * rads.cos() - way.y * rads.sin(), way.x * rads.sin() + way.y * rads.cos()),
            'R' => {
                rads = -rads;
                println!("rads {}", rads);
                way = Vector2D::new(way.x * rads.cos() - way.y * rads.sin(), way.x * rads.sin() + way.y * rads.cos());
            },
            'F' => {
                pos += way * *param;
            },
            _ => panic!(),
        }

    }
    println!("ship at {:?}", pos);
    return (pos.x.abs() + pos.y.abs()) as i32;
}
//Vector2D::new(0_f32, 0_f32);
fn main() {
    let mut reader = std::io::stdin();
    reader.lock();
    let mut input = String::new();
    reader.read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    println!("Result {}", execute(lines));
}
