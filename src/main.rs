use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;
use ringbuf::{Consumer, Producer, RingBuffer};

struct Model {
    locations: Vec<Vec3>,
    camera_pos: Vec3,
    _in_stream: audio::Stream<InputModel>,
    consumer: Consumer<f32>,
    current_note: String,
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
        locations: Vec::with_capacity(8192),
        camera_pos: Vec3::ZERO,
        _in_stream: in_stream,
        consumer: cons,
        current_note: "4".to_owned(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut new_pos = if let Some(pos) = model.locations.last() {
        *pos
    } else {
        Vec3::ZERO
    };

    let mut buf = Vec::with_capacity(1024);
    while !model.consumer.is_empty() {
        let recorded_sample = match model.consumer.pop() {
            Some(f) => f,
            None => 0.0,
        };

        buf.push(recorded_sample);
        if buf.len() == 1024 {
            const SAMPLE_RATE: usize = 44100;
            const SIZE: usize = 1024;
            const PADDING: usize = SIZE / 2;
            const POWER_THRESHOLD: f32 = 5.0;
            const CLARITY_THRESHOLD: f32 = 0.5;

            let mut detector = McLeodDetector::new(SIZE, PADDING);

            if let Some(pitch) = detector.get_pitch(&buf, SAMPLE_RATE, POWER_THRESHOLD, CLARITY_THRESHOLD) {
                let midi = freq_to_midi(pitch.frequency);
                new_pos.x = map_range(pitch.frequency, 200.0, 2000.0, 10.0, -10.0);
                model.current_note = midi_to_tab(midi, "Bb").to_string();
            }
            new_pos.y += 0.01;
            new_pos.z += 0.03;

            if model.locations.len() == model.locations.capacity() {
                model.locations.rotate_left(1);
                model.locations.pop();
            }
            model.locations.push(new_pos);

            buf.clear();
        }
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

fn from_camera_view(point: Vec3, model: &Model) -> Vec2 {
    let point = point - model.camera_pos;
    to_screen_position(&point)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for win in model.locations.windows(2) {
        let mut line_points: [Vec2; 2] = [Vec2::ZERO; 2];
        let mut line_color_points: [Vec3; 2] = [Vec3::ZERO; 2];

        for (i, point) in win.iter().enumerate() {
            line_points[i] = from_camera_view(*point, model);
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

    draw.text(&model.current_note).x(from_camera_view(
        *model.locations.last().unwrap_or(&Vec3::ZERO),
        model,
    )
    .x);
    draw.to_frame(app, &frame).unwrap();
}

struct InputModel {
    pub producer: Producer<f32>,
}

fn pass_in(model: &mut InputModel, buffer: &Buffer) {
    for sample in buffer.frames().map(|f| f[0]) {
        model.producer.push(sample).ok();
    }
}

fn freq_to_midi(freq: f32) -> u8 {
    (12.0 * (freq / 440.0).log2() + 69.0).round() as u8
}

fn midi_to_tab(midi: u8, key: &str) -> &'static str {
    let notes_in_order = [
        "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
        "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
        "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10",
    ];
    let offset = match key {
        "C" => 0,
        "G" => -5,
        "D" => 2,
        "A" => -3,
        "E" => 4,
        "B" => -1,
        "F#" => 6,
        "Db" => 1,
        "Ab" => -4,
        "Eb" => 3,
        "Bb" => -2,
        "F" => 5,
        "LF" => -7,
        "LC" => -12,
        "LD" => -10,
        "HG" => 7,
        _ => {
            panic!()
        }
    };
    let index: isize = midi as isize - 60 - offset;
    if index < 0 || index > notes_in_order.len() as isize - 1 {
        return "";
    }
    notes_in_order[index as usize]
}
