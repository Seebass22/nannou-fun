use nannou::{
    ease::{self, map_clamp},
    prelude::*,
};

struct Model {
    add: f32,
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

    Model { add: 0.0 }
}

fn update(app: &App, model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let mut i = app.time * 128.0;
    let width = 160.0;

    for y in 0..9 {
        for x in 0..16 {
            let x = x as f32 * width - 1280.0 + 0.5 * width;
            let y = y as f32 * width - 720.0 + 0.5 * width;
            let index = fmod(i, 144.0);

            draw.rect().x_y(x, y).w_h(width, width).color(LinSrgb::new(
                model.add + index / 144.0,
                model.add + index / 144.0,
                model.add + index / 144.0,
            ));
            i += 1.0;
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
