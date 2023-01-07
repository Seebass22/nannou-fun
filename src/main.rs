use nannou::prelude::*;

struct Model {
    points: Vec<Vec3>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(1920, 1080).build().unwrap();
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

    Model { points }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for point in model.points.iter_mut() {
        rotate_x(point, 0.001);
        rotate_y(point, 0.002);
        rotate_z(point, 0.003);
    }
    // model.points.sort_by(|a,b| b.z.partial_cmp(&a.z).unwrap());
}

fn rotate_z(point: &mut Vec3, angle: f32) {
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
    draw.background().color(BLACK);

    let mut points: Vec<Vec<Vec2>> = Vec::new();
    let mut points_vec: Vec<Vec2> = Vec::new();
    for point in model.points.iter() {
        let z = 100.0 + point.z;
        let x = point.x / (0.01 * z);
        let y = 15.0 * (2.0 * app.time + 0.1 * x).sin() * 0.3 + point.y / (0.01 * z);

        points_vec.push(Vec2::new(15.0 * x, 15.0 * y));
        if points_vec.len() == 8 {
            points.push(points_vec.clone());
            points_vec.clear();
        }
    }
    for points_vec in points.into_iter() {
        draw.polyline().weight(2.0).points(points_vec)
            .color(WHITE);
    }

    draw.rect()
        .w_h(2000.0, 2000.0)
        .color(srgba(0.0, 0.0, 0.0, 0.3));

    draw.to_frame(app, &frame).unwrap();
}
