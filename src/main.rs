use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{Consumer, Producer, RingBuffer};

struct Model {
    locations: Vec<Vec3>,
    camera_pos: Vec3,
    _in_stream: audio::Stream<InputModel>,
    consumer: Consumer<f32>,
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

    // Initialise the audio host so we can spawn an audio stream.
    let audio_host = audio::Host::new();

    // Create a ring buffer and split it into producer and consumer
    let latency_samples = 8192;
    let ring_buffer = RingBuffer::<f32>::new(latency_samples * 2); // Add some latency
    let (mut prod, cons) = ring_buffer.split();
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        prod.push(0.0).unwrap();
    }

    // Create input model and input stream using that model
    let in_model = InputModel { producer: prod };
    let in_stream = audio_host
        .new_input_stream(in_model)
        .capture(pass_in)
        .build()
        .unwrap();

    in_stream.play().unwrap();

    Model {
        locations: Vec::with_capacity(16384),
        camera_pos: Vec3::ZERO,
        _in_stream: in_stream,
        consumer: cons,
        // out_stream,
    }
}

impl Model {
    fn reset(&mut self) {
        self.locations.clear();
        self.camera_pos = Vec3::ZERO;
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.locations.len() == model.locations.capacity() {
        model.reset();
    }

    let mut new_pos = if let Some(pos) = model.locations.last() {
        *pos
    } else {
        Vec3::ZERO
    };


    let mut recorded_sample;
    let mut count = 0;
    while !model.consumer.is_empty() {
        recorded_sample = match model.consumer.pop() {
            Some(f) => f,
            None => 0.0,
        };
        if count % 10 == 0 {
            new_pos.x = 20.0 * recorded_sample;
            new_pos.y += 0.01;
            new_pos.z += 0.03;
            model.locations.push(new_pos);
        }
        if model.locations.len() == model.locations.capacity() {
            model.reset();
        }
        count += 1;
    }

    let mut direction = new_pos - model.camera_pos;
    direction.x = 0.0;
    model.camera_pos += direction;
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
    draw.background().color(BLACK);

    for win in model.locations.windows(2) {
        let mut line_points: [Vec2; 2] = [Vec2::ZERO; 2];
        let mut line_color_points: [Vec3; 2] = [Vec3::ZERO; 2];

        for (i, point) in win.iter().enumerate() {
            let mut modified_point = *point;
            modified_point -= model.camera_pos;
            line_points[i] = to_screen_position(&modified_point);
            line_color_points[i] = *point;
        }

        let r = map_range((line_color_points[1].z * 0.001).sin(), -1.0, 1.0, 0.1, 1.0);
        let b = map_range((line_color_points[1].y * 0.005).sin(), -1.0, 1.0, 0.1, 1.0);
        let g = map_range((line_color_points[1].x * 0.5).sin(), -1.0, 1.0, 0.1, 1.0);
        if magnitude(&line_points) < 800.0 {
            draw.polyline()
                .weight(0.5)
                .points(line_points)
                .color(srgb(r, g, b));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

struct InputModel {
    pub producer: Producer<f32>,
}

fn pass_in(model: &mut InputModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        for sample in frame {
            model.producer.push(*sample).ok();
        }
    }
}
