#![allow(dead_code)]

use std::time::Duration;

use layer::Layer;
use layers::arm;
use layers::drive;

use sdl2::rect::Point;
use sdl2::{event::Event, pixels::Color};

mod constants;
mod frc;
mod layer;
mod layers;

fn main() {
    let mut inst = nt::NetworkTableInstance::get_default();
    inst.start_client_3("Thunderstorm");
    if !constants::SIMULATION {
        inst.set_server_team(509, 1735);
    } else {
        inst.set_server("127.0.0.1", 1735);
    }
    inst.start_driver_station_client(1735);

    std::thread::sleep(Duration::from_secs(1));

    if !inst.is_connected() {
        println!("Failed to connect to NetworkTables, check if the robot is on or if the simulator is running!");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Thunderstorm Interface", 1280, 720)
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

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut drive_layer = drive::Drive::create(&texture_creator, Point::new(580, 120));
    let mut operator_layer = arm::Arm::create(&texture_creator, Point::new(47, 274));

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

        canvas.clear();
        operator_layer.render(&mut canvas, &inst);
        drive_layer.render(&mut canvas, &inst);
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
