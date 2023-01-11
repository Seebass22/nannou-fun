use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;

struct Audio {
    sounds: Vec<audrey::read::BufFileReader>,
}

struct Model {
    points: Vec<Vec3>,
    stream: audio::Stream<Audio>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(1920, 1080).build().unwrap();
    let mut points = Vec::new();

    for x in (-24..=24).step_by(3) {
        for y in (-24..=24).step_by(3) {
            for z in (-24..=24).step_by(3) {
                let x = x as f32;
                let y = y as f32;
                let z = z as f32;
                points.push(vec3(x, y, z));
            }
        }
    }

    // Initialise the audio host so we can spawn an audio stream.
    let audio_host = audio::Host::new();

    // Initialise the state that we want to live on the audio thread.
    let sounds = vec![];
    let model = Audio { sounds };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();

    stream.play().unwrap();

    let sound = audrey::open("Chemical.wav").expect("failed to load sound");
    stream
        .send(move |audio| {
            audio.sounds.push(sound);
        })
        .ok();

    Model { points, stream }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play the audio file.
fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let mut have_ended = vec![];
    let len_frames = buffer.len_frames();

    // Sum all of the sounds onto the buffer.
    for (i, sound) in audio.sounds.iter_mut().enumerate() {
        let mut frame_count = 0;
        let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);
        for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
            for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
                *sample += *file_sample;
            }
            frame_count += 1;
        }

        // If the sound yielded less samples than are in the buffer, it must have ended.
        if frame_count < len_frames {
            have_ended.push(i);
        }
    }

    // Remove all sounds that have ended.
    for i in have_ended.into_iter().rev() {
        audio.sounds.remove(i);
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for point in model.points.iter_mut() {
        rotate_x(point, 0.001);
        rotate_y(point, 0.002);
        rotate_z(point, 0.003);
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

    let x_scale = map_range(app.mouse.x, 0.0, 2000.0, 0.0, 1.0);
    let y_scale = map_range(app.mouse.y, 0.0, 2000.0, 0.0, 1.0);

    let mut points: Vec<Vec<Vec2>> = Vec::new();
    let mut points_vec: Vec<Vec2> = Vec::new();
    for point in model.points.iter() {
        let z = map_range(app.time.sin(), -1.0, 1.0, 0.4, 1.5) * 100.0 + point.z;
        let _x = point.x / (0.01 * z);
        let x = 15.0 * (3.0 * app.time + 0.2 * _x).sin() * x_scale + point.x / (0.01 * z);
        let y = 15.0 * (3.0 * app.time + 0.2 * x).sin() * y_scale + point.y / (0.01 * z);

        points_vec.push(Vec2::new(10.0 * x, 10.0 * y));
        if points_vec.len() == 17 {
            points.push(points_vec.clone());
            points_vec.clear();
        }
    }

    for points_vec in points.into_iter() {
        let x = points_vec.get(0).unwrap().x;
        let y = points_vec.get(0).unwrap().y;

        let r = (app.time + 0.005 * x).sin();
        let g = map_range(x, -1000.0, 1000.0, 0.0, 1.0);
        let b = map_range(y, -1000.0, 1000.0, 0.0, 1.0);
        draw.polyline().weight(2.0).points(points_vec)
            .color(srgb(r, g, b));
    }

    draw.to_frame(app, &frame).unwrap();
}
