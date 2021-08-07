#![allow(unreachable_code, unused_variables, dead_code)]
use nannou::prelude::*;
use nannou::noise::*;

struct Model {
    points: Vec<Vec2>,
    noise: Perlin,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut points = Vec::new();
    for y in -50..50 {
        for x in -50..50 {
            let x = x as f32;
            let y = y as f32;
            points.push(vec2(x * 10.0, y * 10.0));
        }
    }

    let noise = Perlin::new();
    Model {
        points,
        noise,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32;

    // if ! model.points.is_empty() {
    //     model.points.remove(0);
    // }
    let sn = 0.01;

    for point in model.points.iter_mut() {
        point.x += point.x * 0.0005 * (point.y * 0.3 * time.sin() + 1.0)
            * model.noise.get([sn*point.x as f64, sn*point.y as f64]) as f32;
            // * random::<f32>();
        point.y += point.y * 0.0005 * (point.x * 0.3 * time.sin() + 1.0)
            * model.noise.get([sn*point.x as f64, sn*point.y as f64]) as f32;

        point.x += 0.2 * point.y.sin();

        // point.x += point.x * 0.00003 * time;
        // point.y += point.y * 0.00003 * time;
        if time < 300.0 {
            point.x += point.x * 0.002;
            point.y += point.y * 0.002;
        } else if time < 600.0 {
            point.x += point.y * 0.002;
            point.y += point.x * 0.002;
        } else if time < 900.0 {
            point.x += point.x * 0.003;
            point.y += point.x * 0.003;
        } else if time < 1200.0 {
            point.x += point.x * 0.0005;
            point.y += point.y * 0.0005;
        } else if time < 1500.0 {
            point.x -= point.x * 0.01;
            point.y -= point.y * 0.01;
        } else if time < 1800.0 {
            point.x += point.x * point.x.sin() * 0.01;
            point.y += point.y * point.y.cos() * 0.01;
        } else if time < 2100.0 {
            point.x += point.x * 0.005;
            point.y += point.y * 0.005;

            point.x += point.y * 0.0005;
            point.y += point.x * 0.0005;
        } else {
            point.x += 0.5 * point.y.sin();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let time = (app.elapsed_frames() / 50) as f32;
    let timeval = (time.sin() + 1.0) / 2.0;

    draw.rect().w_h(2000.0, 2000.0).color(srgba(0.0, 0.0, 0.0, 0.02));

    for point in model.points.iter() {
        draw.ellipse().x_y(point.x, point.y).w_h(2.0, 2.0)
            // .color(srgb(timeval, timeval, 0.0));
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

