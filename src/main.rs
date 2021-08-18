use nannou::prelude::*;
use nannou::noise::*;
use nannou::rand::random_range;

struct Model {
    points: Vec<Vec2>,
    noise: Perlin,
    radius: f32,
    scale: f64,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(1920, 1080).build().unwrap();
    let mut points = Vec::new();

    let radius = 20.0;
    let noise = Perlin::new();
    for _i in 0..400 {
        let x = 0.0;
        let y = 0.0;
        points.push(vec2(x, y));
    }

    Model {
        points,
        noise,
        radius,
        scale: 0.01,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let sn = model.scale;
    let speed = 0.08;

    for point in model.points.iter_mut() {
        point.x += point.x * 0.015
            * model.noise.get([sn*point.x as f64, sn*point.y as f64]) as f32;
        point.y += point.y * 0.015
            * model.noise.get([sn*point.x as f64, sn*point.y as f64]) as f32;

        point.x += point.x * speed;
        point.y += point.y * speed;
    }

    if app.elapsed_frames() % 200 == 0 {
        for i in 0..100 {
            let val = (-model.radius / 2.0) + (i as f32 * model.radius / 100.0);
            let max = model.radius / 2.0;

            set_point(val, max, i, model);
            set_point(max, -val, i+100, model);
            set_point(-val, -max, i+200, model);
            set_point(-max, val, i+300, model);
        }
        model.scale = random_range(0.0001, 0.01);
    }
}

fn set_point(x: f32, y: f32, i: usize, model: &mut Model) {
    let point = model.points.get_mut(i).unwrap();
    point.x = x;
    point.y = y;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let mut points = model.points.clone();
    points.push(*points.get(0).unwrap());
    draw.polyline().points(points).color(WHITE);

    draw.rect().w_h(2000.0, 2000.0).color(srgba(0.0, 0.0, 0.0, 0.1));

    draw.to_frame(app, &frame).unwrap();
}

