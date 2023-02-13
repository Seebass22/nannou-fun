use nannou::prelude::*;
use rand::prelude::*;

struct Model {
    locations: Vec<Vec3>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(2560, 1440)
        .build()
        .unwrap();
    Model {
        locations: Vec::with_capacity(1000),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut new_pos = if let Some(pos) = model.locations.last() {
        *pos
    } else {
        Vec3::ZERO
    };
    step(&mut new_pos);
    model.locations.push(new_pos);
}

fn step(pos: &mut Vec3) {
    let mut rng = thread_rng();
    let rand: u8 = rng.gen();
    let distance = 0.2;
    let dir = if rand % 2 == 0 { 1.0 } else { -1.0 };
    match rand % 3 {
        0 => {
            pos.x += dir * distance;
        }
        1 => {
            pos.y += dir * distance;
        }
        2 => {
            pos.z += dir * distance;
        }
        _ => {
            panic!()
        }
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

fn to_screen_position(point: &Vec3) -> Vec2 {
    let z = point.z - 10.0;
    let x = point.x / (0.01 * z);
    let y = point.y / (0.01 * z);
    Vec2::new(10.0 * x, 10.0 * y)
}

fn magnitude(points: &[Vec2]) -> f32 {
    let inner: f32 = (points[1].x - points[0].x).pow(2) + (points[1].y - points[0].y).pow(2);
    inner.sqrt()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let last_point = if let Some(point) = model.locations.last() {
        *point
    } else {
        Vec3::ZERO
    };

    for win in model.locations.windows(2) {
        let mut line_points: [Vec2; 2] = [Vec2::ZERO; 2];
        let mut line_color_points: [Vec3; 2] = [Vec3::ZERO; 2];

        for (i, point) in win.iter().enumerate() {
            let mut modified_point = *point;
            modified_point -= last_point;
            line_points[i] = to_screen_position(&modified_point);
            line_color_points[i] = *point;
        }

        let r = map_range(line_color_points[1].x, -5.0, 5.0, 0.1, 1.0);
        let g = map_range(line_color_points[1].y, -5.0, 5.0, 0.1, 1.0);
        let b = map_range(line_color_points[1].z, -1.0, 1.0, 0.1, 1.0);

        if magnitude(&line_points) < 500.0 {
            draw.polyline()
                .weight(2.0)
                .points(line_points)
                .color(srgb(r, g, b));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
