use std::{collections::HashMap, hash::Hash};

// use nt::EntryValue;
use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
};

use ::nt::*;

use crate::constants::MODULE_LOCATIONS;

mod constants;

fn draw_arrow<T>(canvas: &mut Canvas<T>, start: Point, end: Point, color: Color)
where
    T: sdl2::render::RenderTarget,
{
    let dx = (end.x - start.x) as f64;
    let dy = (end.y - start.y) as f64;
    let angle = dy.atan2(dx);
    // let length = (dx * dx + dy * dy).sqrt() as i32;

    let arrow_width = 10;
    let arrow_length = 20;
    let arrow_tip = Point::new(
        end.x - (arrow_length as f64 * angle.cos()) as i32,
        end.y - (arrow_length as f64 * angle.sin()) as i32,
    );
    let arrow_left = Point::new(
        arrow_tip.x + (arrow_width as f64 * (angle + std::f64::consts::PI / 6.0).cos()) as i32,
        arrow_tip.y + (arrow_width as f64 * (angle + std::f64::consts::PI / 6.0).sin()) as i32,
    );
    let arrow_right = Point::new(
        arrow_tip.x + (arrow_width as f64 * (angle - std::f64::consts::PI / 6.0).cos()) as i32,
        arrow_tip.y + (arrow_width as f64 * (angle - std::f64::consts::PI / 6.0).sin()) as i32,
    );
    canvas.draw_line(start, end).unwrap();
    canvas
        .filled_trigon(
            arrow_left.x() as _,
            arrow_left.y() as _,
            arrow_right.x() as _,
            arrow_right.y() as _,
            arrow_tip.x() as _,
            arrow_tip.y() as _,
            color,
        )
        .unwrap();
}

fn main() {
    let mut inst = nt::NetworkTableInstance::get_default();
    inst.start_client_3("Thunderstorm");
    if !constants::SIMULATION {
        inst.set_server_team(509, 1735);
    } else {
        inst.set_server("127.0.0.1", 1735);
    }
    inst.start_driver_station_client(1735);

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

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    let chassis_image = texture_creator.load_texture("chassis.png").unwrap();
    let wheel_image = texture_creator.load_texture("wheel.png").unwrap();

    let chassis_src = Rect::new(
        0,
        0,
        chassis_image.query().width,
        chassis_image.query().height,
    );
    let chassis_dst = Rect::new(100, 0, chassis_src.width(), chassis_src.height());

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
        canvas
            .copy(&chassis_image, chassis_src, chassis_dst)
            .unwrap();
        for module_number in 0..4 {
            // Get the steer angle and velocity for the wheel.
            let angle = 360.0 - inst
                .get_entry(format!("/Thunderstorm/Module{}Angle", module_number).as_str())
                .get_value()
                .unwrap()
                .get_double()
                .unwrap_or(0.0);
            let velocity_mps = inst
                .get_entry(format!("/Thunderstorm/Module{}Velocity", module_number).as_str())
                .get_value()
                .unwrap()
                .get_double()
                .unwrap_or(0.0);
            let location = MODULE_LOCATIONS[module_number];
            // Render the wheel.
            let wheel_src = Rect::new(0, 0, wheel_image.query().width, wheel_image.query().height);
            let wheel_dst = Rect::new(
                location.0,
                location.1,
                wheel_src.width(),
                wheel_src.height(),
            );
            let center = Point::new(wheel_dst.width() as i32 / 2, wheel_dst.height() as i32 / 2);
            canvas
                .copy_ex(
                    &wheel_image,
                    Some(wheel_src),
                    Some(wheel_dst),
                    angle,
                    center,
                    false,
                    false,
                )
                .unwrap();
            if velocity_mps != 0.0_f64 {
                let mut magnitude = velocity_mps / constants::MAX_SPEED * 100.0;
                if angle >= 90.0 {
                    magnitude = -magnitude;
                }
                let src_point = Point::new(location.0, location.1).offset(20, 46);
                let dst_point = src_point.offset(
                    (magnitude * angle.to_radians().cos()).round() as i32,
                    (magnitude * angle.to_radians().sin()).round() as i32,
                );
                draw_arrow(&mut canvas, src_point, dst_point, Color::RGB(0, 0, 0));
                canvas.set_draw_color(Color::RGB(255, 255, 255))
            }
        }
        canvas.present();
    }
}
