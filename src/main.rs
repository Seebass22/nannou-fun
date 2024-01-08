use nannou::{prelude::*, ease::{self, map_clamp}};

struct Model {
    locations: Vec<Vec2>,
    left_click_time: f32,
    right_click_time: f32,
    y: f32,
    x: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(2560, 1440)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let mut locations = Vec::new();
    for x in -1000..=1000 {
        locations.push(Vec2::new(x as f32, 0.0));
    }

    Model {
        left_click_time: -10.0,
        right_click_time: -10.0,
        y: 1.0,
        x: 1.0,
        locations,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time_since_left_click = app.time - model.left_click_time;
    let left_fac = model.y * map_clamp(time_since_left_click, 0.0, 0.3, 0.0, 1.0, ease::bounce::ease_out);
    for point in model.locations.iter_mut() {
        let time_scale = 20.0 * model.x;
        let x_scale = 0.02;
        let y_scale = 150.0 * left_fac;
        point.y = y_scale * (x_scale * point.x + time_scale * app.time).sin();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.polyline()
        .points(model.locations.clone())
        .color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.left_click_time = app.time;
        model.y = map_range(app.mouse.y, -720.0, 720.0, 0.1, 5.0);
        model.x = map_range(app.mouse.x, -1280.0, 1280.0, 0.1, 5.0);
    } else {
        model.right_click_time = app.time;
    }
}
