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

    let radius = 1.0;
    let noise = Perlin::new();
    for i in 0..360 {
        for i2 in 0..10 {
            let x = radius * deg_to_rad(i as f32 + 0.1 * i2 as f32).cos();
            let y = radius * deg_to_rad(i as f32 + 0.1 * i2 as f32).sin();
            points.push(vec2(x, y));
        }
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
        for i in 0..360 {
            for i2 in 0..10 {
                let x = model.radius * deg_to_rad(i as f32 + 0.1 * i2 as f32).cos();
                let y = model.radius * deg_to_rad(i as f32 + 0.1 * i2 as f32).sin();
                let point = model.points.get_mut(i*10 + i2).unwrap();
                point.x = x;
                point.y = y;
            }
        }
        model.scale = random_range(0.0001, 0.01);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let pointsize = 3.0;
    for point in model.points.iter() {
        draw.ellipse().x_y(point.x, point.y).w_h(pointsize, pointsize).color(WHITE);
    }

    draw.rect().w_h(2000.0, 2000.0).color(srgba(0.0, 0.0, 0.0, 0.1));

    draw.to_frame(app, &frame).unwrap();
}

