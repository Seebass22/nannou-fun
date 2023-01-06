use nannou::prelude::*;
use rand::prelude::*;

struct Model {
    points: Vec<Vec3>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(640, 480).build().unwrap();
    let mut points = Vec::new();

    for x in (-24..24).step_by(6) {
        for y in (-24..24).step_by(6) {
            for z in (-24..24).step_by(6) {
                let x = x as f32;
                let y = y as f32;
                let z = z as f32;
                points.push(vec3(x, y, z));
            }
        }
    }

    let mut rng = rand::thread_rng();
    points.shuffle(&mut rng);
    Model { points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let m_x = app.mouse.x;
    let m_y = app.mouse.y;
    for point in model.points.iter_mut() {
        rotate_x(point, 0.00001 * m_y);
        rotate_y(point, 0.00001 * m_x);
        // _rotate_z(point, 0.003);
    }
}

fn _rotate_z(point: &mut Vec3, angle: f32) {
    let s = angle.sin();
    let c = angle.cos();
    point.x = point.x * c - point.y * s;
    point.y = point.x * s + point.y * c;
}

fn rotate_x(point: &mut Vec3, angle: f32) {
    let s = angle.sin();
    let c = angle.cos();
    point.y = point.y * c - point.z * s;
    point.z = point.y * s + point.z * c;
}

fn rotate_y(point: &mut Vec3, angle: f32) {
    let s = angle.sin();
    let c = angle.cos();
    point.x = point.x * c - point.z * s;
    point.z = point.x * s + point.z * c;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let size = 5.0;
    let mut points: Vec<Vec2> = Vec::new();
    for point in model.points.iter() {
        let z = 100.0 + point.z;
        let x = point.x / (0.01 * z);
        let y = point.y / (0.01 * z);
        // let x = point.x + 0.5 * point.z;
        // let y = point.y + 0.5 * point.z;

        points.push(Vec2::new(size * x, size * y));
    }

    let mut rng = rand::thread_rng();
    points.shuffle(&mut rng);
    draw.polyline().weight(0.2).points(points).color(WHITE);

    draw.rect()
        .w_h(2000.0, 2000.0)
        .color(srgba(0.0, 0.0, 0.0, 0.3));

    draw.to_frame(app, &frame).unwrap();
}
