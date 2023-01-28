use nannou::{prelude::*, math::ConvertAngle};
use nannou_osc as osc;

const PORT: u16 = 9000;

struct Model {
    points: Vec<Vec3>,
    time_start: f32,
    active_left: bool,
    key_offset: usize,
    bounce_val: f32,
    receiver: osc::Receiver,
    touch_pos: Vec2,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.time_start = app.time;
        model.active_left = true;
        model.bounce_val += 10.0;
    } else {
        model.time_start = app.time;
        model.active_left = false;
        model.bounce_val += 30.0;
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    let keys = [
        Key::Q, Key::W, Key::E, Key::R, Key::T, Key::Y, Key::U, Key::I, Key::O, Key::P,
    ];

    let pos = keys.iter().position(|k| *k == key);

    model.key_offset = if let Some(pos) = pos {
            pos
    } else {
        0
    };
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(1920, 1080)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let mut points = Vec::new();

    let receiver = osc::receiver(PORT).unwrap();

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

    Model {
        points,
        time_start: 0.0,
        active_left: true,
        key_offset: 0,
        bounce_val: 0.0,
        receiver,
        touch_pos: Vec2::new(0.0, 0.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for point in model.points.iter_mut() {
        rotate_x(point, 0.001);
        rotate_y(point, 0.002);
        rotate_z(point, 0.003);
    }
    model.bounce_val *= 0.967;

    let mut should_bounce = false;
    for (packet, _addr) in model.receiver.try_iter() {
        dbg!(&packet);
        if let osc::Packet::Message(msg) = packet.clone() {
            match msg.args {
                Some(args) => {
                    if let [x, y] = &args[..] {
                        if let osc::Type::Float(x) = x {
                            if *x != -1.0 {
                                model.touch_pos.x = *x;
                            } else {
                                should_bounce = true;
                            }
                        }
                        if let osc::Type::Float(y) = y {
                            if *y != -1.0 {
                                model.touch_pos.y = *y;
                            } else {
                                should_bounce = true;
                            }
                        }
                    }
                },
                _ => { panic!() },
            }
        }

        if model.bounce_val < 100.0 && should_bounce {
            model.time_start = _app.time;
            model.active_left = true;
            model.bounce_val += model.touch_pos.y * 5.0;
        }
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
    let y_scale = model.bounce_val;

    let mut points: Vec<(Vec<Vec2>, Vec<Vec3>)> = Vec::new();
    let mut points_vec = Vec::new();
    let mut colors_vec = Vec::new();

    let periods = (model.bounce_val) +  20.0;
    for (i, point) in model.points.clone().iter_mut().enumerate() {
        let i = (i % 361) as f32;
        rotate_x(point, app.time.sin());

        let wave_value = 0.2 * (i.deg_to_rad() * periods).sin();
        *point *= 4.0 * map_range(y_scale * wave_value, -1.0, 1.0, 1.0, 1.2);

        let z = point.z - 10.0;
        let x = point.x / (0.01 * z);
        let y = point.y / (0.01 * z);

        points_vec.push(Vec2::new(10.0 * x, 10.0 * y));
        colors_vec.push(*point);
        if points_vec.len() == 361 {
            points.push((points_vec.clone(), colors_vec.clone()));
            points_vec.clear();
            colors_vec.clear();
        }
    }

    for (i, (points_vec, colors_vec)) in points.into_iter().enumerate() {
        let x = colors_vec[0].x;
        let y = colors_vec[0].y;

        let time_since_click = app.time - model.time_start;
        let show = (0.05 * i as f32 + 5.0 * time_since_click).sin();

        let mult = if show > 0.9 && time_since_click < 1.0 {
            1.0
        } else {
            0.0
        };

        let r = mult * model.touch_pos.x;

        let g = mult * map_range(x, -1.0, 1.0, 0.0, 1.0);
        let b = mult * map_range(y, -1.0, 1.0, 0.0, 1.0);
        if r > 0.2 || g > 0.2 || b > 0.2 {
        draw.polyline().weight(12.0).points(points_vec)
            .color(srgb(r, g, b));
        }
    };

    draw.to_frame(app, &frame).unwrap();
}
