use nannou::prelude::*;
use rand::prelude::*;

struct Model {
    old_location: Vec3,
    location: Vec3,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(1920, 1080).build().unwrap();
    Model {
        old_location: Vec3::ZERO,
        location: Vec3::ZERO,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.old_location = model.location;
    step(&mut model.location);
}

fn step(pos: &mut Vec3) {
    let mut rng = thread_rng();
    let rand: u8 = rng.gen();
    let distance = 0.1;
    let dir = if rand % 2 == 0 { 1.0 } else { -1.0 };
    match rand % 3 {
        0 => {
            pos.x += dir * distance;
        },
        1 => {
            pos.y += dir * distance;
        },
        2 => {
            pos.z += dir * distance;
        },
        _ => { panic!() },
    }
}

fn _rotate_z(point: &mut Vec3, angle: f32) {
    let s = angle.sin();
    let c = angle.cos();
    point.x = point.x * c - point.y * s;
    point.y = point.x * s + point.y * c;
}

fn _rotate_x(point: &mut Vec3, angle: f32) {
    let s = angle.sin();
    let c = angle.cos();
    point.y = point.y * c - point.z * s;
    point.z = point.y * s + point.z * c;
}

fn _rotate_y(point: &mut Vec3, angle: f32) {
    let s = angle.sin();
    let c = angle.cos();
    point.x = point.x * c - point.z * s;
    point.z = point.x * s + point.z * c;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    }

    let mut line_points: [Vec2; 2] = [Vec2::ZERO; 2];
    let mut line_color_points: [Vec3; 2] = [Vec3::ZERO; 2];
    for (i, point) in [model.old_location, model.location].iter().enumerate() {
        let z = point.z - 10.0;
        let x = point.x / (0.01 * z);
        let y = point.y / (0.01 * z);
        line_points[i] = Vec2::new(10.0 * x, 10.0 * y);
        line_color_points[i] = *point;
    }

    let r = 1.0;
    draw.polyline().weight(2.0).points(line_points)
        .color(srgb(r, 0.0, 0.0));

    draw.to_frame(app, &frame).unwrap();
}
