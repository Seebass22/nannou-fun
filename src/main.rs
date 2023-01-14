use nannou::{prelude::*, math::ConvertAngle};

struct Model {
    points: Vec<Vec3>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(1920, 1080).build().unwrap();
    let mut points = Vec::new();

    let radius = 1.0;
    for theta in (0..=180).step_by(10) {
        for phi in (0..=360).step_by(1) {
            let theta = (theta as f32).deg_to_rad();
            let phi = (phi as f32).deg_to_rad();
            let x = radius * theta.sin() * phi.cos();
            let y = radius * theta.sin() * phi.sin();
            let z = radius * theta.cos();
            points.push(vec3(x, y, z));
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

    let y_scale = map_range(app.mouse.y, 0.0, 2000.0, 0.5, 8.0);

    let mut points: Vec<(Vec<Vec2>, Vec<Vec3>)> = Vec::new();
    let mut points_vec = Vec::new();
    let mut colors_vec = Vec::new();

    for point in model.points.clone().iter_mut() {
        rotate_x(point, app.time.sin());
        *point *= map_range((5.5 * app.time).sin(), -1.0, 1.0, 1.0, 5.0);

        let z = point.z - 10.0;
        let x = point.x / (0.01 * z);
        let y = (y_scale * x).sin() + point.y / (0.01 * z);

        points_vec.push(Vec2::new(10.0 * x, 10.0 * y));
        colors_vec.push(*point);
        if points_vec.len() == 361 {
            points.push((points_vec.clone(), colors_vec.clone()));
            points_vec.clear();
            colors_vec.clear();
        }
    }

    for (points_vec, colors_vec) in points.into_iter() {
        let x = colors_vec[0].x;
        let y = colors_vec[0].y;
        let z = colors_vec[0].z;

        let r = map_range((2.0 * app.time + 0.005 * x).sin() * z, -1.0, 1.0, 0.0, 1.0);
        let g = map_range(x, -1.0, 1.0, 0.0, 1.0);
        let b = map_range(y, -1.0, 1.0, 0.0, 1.0);
        draw.polyline().weight(1.0).points(points_vec)
            .color(srgb(r, g, b));
    }

    draw.to_frame(app, &frame).unwrap();
}
