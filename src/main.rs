use std::collections::HashMap;

use nt::EntryValue;
use sdl2::{
    event::Event,
    image::LoadTexture,
    rect::{Point, Rect},
};

use crate::constants::MODULE_LOCATIONS;

mod constants;

#[tokio::main]
async fn main() {
    let client = if constants::SIMULATION {
        nt::NetworkTables::connect("127.0.0.1:1735", "thunderstorm")
            .await
            .expect("Failed to connect to NetworkTables, check if the simulator is running...")
    } else {
        nt::NetworkTables::connect("10.50.9.2:1735", "thunderstorm")
            .await
            .expect("Failed to connect to NetworkTables, check if the robot is on...")
    };

    client.add_connection_callback(nt::ConnectionCallbackType::ClientDisconnected, |_| {
        println!("Client has disconnected from the server");
    });

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Thunderstorm Interface", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    let chassis_image = texture_creator.load_texture("chassis.png").unwrap();
    let wheel_image = texture_creator.load_texture("wheel.png").unwrap();

    let mut map = HashMap::new();

    let src_rect = Rect::new(
        0,
        0,
        chassis_image.query().width,
        chassis_image.query().height,
    );
    let dst_rect = Rect::new(100, 0, src_rect.width(), src_rect.height());

    'running: loop {
        // SDL2 Event handler.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Collect data from NetworkTables.
        for (id, data) in client.entries() {
            if data.name.starts_with("/Thunderstorm") {
                match data.value {
                    EntryValue::Double(value) => {
                        map.entry(data.name).or_insert(value);
                    }
                    _ => {}
                }
            }
        }

        canvas.clear();
        canvas.copy(&chassis_image, src_rect, dst_rect).unwrap();
        for module_number in 0..4 {
            // Get the steer angle for the wheel.
            let angle = *map
                .get(format!("/Thunderstorm/Module{}Angle", module_number).as_str())
                .unwrap_or(&45.0_f64);
            let location = MODULE_LOCATIONS[module_number];
            // Define the source and destination rectangles
            let src_rect = Rect::new(0, 0, wheel_image.query().width, wheel_image.query().height);
            let dst_rect = Rect::new(location.0, location.1, src_rect.width(), src_rect.height());

            let center = Point::new(dst_rect.width() as i32 / 2, dst_rect.height() as i32 / 2);

            // Render the image texture at the angle
            canvas
                .copy_ex(
                    &wheel_image,
                    Some(src_rect),
                    Some(dst_rect),
                    angle,
                    center,
                    false,
                    false,
                )
                .unwrap();
        }
        canvas.present();
    }
}
