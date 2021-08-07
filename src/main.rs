#![allow(unreachable_code, unused_variables, dead_code)]
use nannou::prelude::*;

struct Model {
    points: Vec<Vec2>
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut points = Vec::new();
    for y in -50..50 {
        for x in -50..50 {
            points.push(vec2(x as f32 * 10.0, y as f32 * 10.0));
        }
    }
    Model { points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32;

    if ! model.points.is_empty() {
        model.points.remove(0);
    }
    for point in model.points.iter_mut() {
        point.x += point.x * 0.0005 * (point.y * 0.3 * time.sin() + 1.0);
        point.y += point.y * 0.0005 * (point.x * 0.3 * time.sin() + 1.0);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect().w_h(2000.0, 2000.0).color(srgba(0.0, 0.0, 0.0, 0.1));

    for point in model.points.iter() {
        draw.ellipse().x_y(point.x, point.y).w_h(3.0, 3.0).color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

