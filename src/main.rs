use nannou::prelude::*;
use rand::prelude::*;

struct Model {
    locations: Vec<Vec3>,
    camera_pos: Vec3,
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
        locations: Vec::with_capacity(1024),
        camera_pos: Vec3::ZERO,
    }
}

impl Model {
    fn reset(&mut self) {
        self.locations.clear();
        self.camera_pos = Vec3::ZERO;
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.locations.len() == model.locations.capacity() {
        model.reset();
    }

    let mut new_pos = if let Some(pos) = model.locations.last() {
        *pos
    } else {
        Vec3::ZERO
    };

    if app.elapsed_frames() % 5 == 0 {
        step(&mut new_pos);
        model.locations.push(new_pos);
    }

    rotate_y(&mut new_pos, app.time);
    let direction = new_pos - model.camera_pos;
    model.camera_pos += 0.05 * direction;
}

fn step(pos: &mut Vec3) {
    let mut rng = thread_rng();
    let rand: u8 = rng.gen();
    let dir = if rand % 2 == 0 { 1.0 } else { -1.0 };
    match rand % 3 {
        0 => {
            pos.x += dir;
        }
        1 => {
            pos.y += dir;
        }
        2 => {
            pos.z += dir;
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

fn rotate_y(point: &mut Vec3, angle: f32) {
    let s = angle.sin();
    let c = angle.cos();
    point.x = point.x * c - point.z * s;
    point.z = point.x * s + point.z * c;
}

fn to_screen_position(point: &Vec3) -> Vec2 {
    let z = point.z;
    let x = point.x + (0.4 * z);
    let y = point.y + (0.4 * z);
    Vec2::new(50.0 * x, 50.0 * y)

    // let z = point.z - 10.0;
    // let x = point.x / (0.01 * z);
    // let y = point.y / (0.01 * z);
    // Vec2::new(10.0 * x, 10.0 * y)
}

fn magnitude(points: &[Vec2]) -> f32 {
    let inner: f32 = (points[1].x - points[0].x).pow(2) + (points[1].y - points[0].y).pow(2);
    inner.sqrt()
}

fn rot(point: &Vec3) -> Vec3 {
    Vec3::new(point.y, -point.x, point.z)
}

fn draw_cube(pos: Vec3, model: &Model, draw: &Draw) {
    let mut points: [Vec3; 2] = [Vec3::ZERO; 2];
    let mut last_point = pos;
    let mut rotation_vec = Vec3::new(0.0, 1.0, 0.0);
    for _i in 0..4 {
        points[0] = last_point;
        points[1] = last_point + rotation_vec;
        rotation_vec = rot(&rotation_vec);
        draw_line(&points, model, draw);
        last_point = points[1];
    }
}

fn draw_line(points: &[Vec3], model: &Model, draw: &Draw) {
    let mut line_points: [Vec2; 2] = [Vec2::ZERO; 2];
    let mut line_color_points: [Vec3; 2] = [Vec3::ZERO; 2];
    for (i, point) in points.iter().enumerate() {
        let mut modified_point = *point;
        modified_point -= model.camera_pos;
        line_points[i] = to_screen_position(&modified_point);
        line_color_points[i] = *point;
    }
    let r = map_range(line_color_points[1].x, -50.0, 50.0, 0.1, 1.0);
    let g = map_range(line_color_points[1].y, -50.0, 50.0, 0.1, 1.0);
    let b = map_range(line_color_points[1].z, -10.0, 10.0, 0.1, 1.0);

    if magnitude(&line_points) < 800.0 {
        draw.polyline()
            .weight(2.0)
            .points(line_points)
            .color(srgb(r, g, b));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for point in model.locations.iter() {
        let mut point = *point;
        rotate_y(&mut point, app.time);

        draw_cube(point, model, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
