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

    for x in (-24..=24).step_by(6) {
        for y in (-24..=24).step_by(6) {
            for z in (-240..=240).step_by(1) {
                let x = x as f32;
                let y = y as f32;
                let z = z as f32;
                points.push(vec3(x, y, 0.1 * z));
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
    draw.background().color(BLACK);

    let x_scale = map_range(app.mouse.x, 0.0, 2000.0, 0.0, 1.0);
    let y_scale = map_range(app.mouse.y, 0.0, 2000.0, 0.0, 1.0);

    let mut points: Vec<(Vec<Vec2>, Vec<Vec3>)> = Vec::new();
    let mut points_vec: Vec<Vec2> = Vec::new();
    let mut colors_vec: Vec<Vec3> = Vec::new();

    for point in model.points.clone().iter() {
        let z = map_range(app.time.sin(), -1.0, 1.0, 0.4, 1.5) * 100.0 + point.z;
        let _x = point.x / (0.01 * z);
        let x = 15.0 * (3.0 * app.time + (app.time * 0.1).sin() * 0.5 * _x).sin() * x_scale + point.x / (0.01 * z);
        let y = 15.0 * (3.0 * app.time + 0.2 * x).sin() * y_scale + point.y / (0.01 * z);

        points_vec.push(Vec2::new(10.0 * x, 10.0 * y));
        colors_vec.push(*point);
        if points_vec.len() == 481 {
            points.push((points_vec.clone(), colors_vec.clone()));
            points_vec.clear();
            colors_vec.clear();
        }
    }

    for (points_vec, colors_vec) in points.into_iter() {
        let x = colors_vec[0].x;
        let y = colors_vec[0].y;
        let z = colors_vec[0].z;

        let r = map_range((2.0 * app.time + 0.005 * x).sin() * z, -30.0, 30.0, 0.0, 1.0);
        let g = map_range(x, -30.0, 30.0, 0.0, 1.0);
        let b = map_range(y, -30.0, 30.0, 0.0, 1.0);
        draw.polyline().weight(2.0).points(points_vec)
            .color(srgb(r, g, b));
    }

    draw.to_frame(app, &frame).unwrap();
}
