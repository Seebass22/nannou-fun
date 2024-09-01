use nannou::{
    prelude::*,
    rand::{thread_rng, Rng},
};

struct Model {
    indices: [u8; 16],
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

    Model { indices: [0u8; 16] }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 10 == 0 {
        let mut rng = thread_rng();
        let mut indices = [0u8; 16];
        for i in indices.iter_mut() {
            *i = rng.gen_range(0..144);
        }
        model.indices = indices;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let mut i = 0.0;
    let index_add = app.time * 128.0;

    let width = 160.0;

    for y in 0..9 {
        for x in 0..16 {
            let x = x as f32 * width - 1280.0 + 0.5 * width;
            let y = y as f32 * width - 720.0 + 0.5 * width;
            let index = fmod(i + index_add, 144.0);

            let mut add = 0.0;
            if model.indices.contains(&(i as u8)) {
                add = 1.0;
            }
            draw.rect().x_y(x, y).w_h(width, width).color(LinSrgb::new(
                add + index / 144.0,
                0.5 * add + index / 144.0,
                0.1 * add + index / 144.0,
            ));
            i += 1.0;
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
