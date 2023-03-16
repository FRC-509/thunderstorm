use nalgebra::Vector2;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, TextureCreator, Canvas}, video::{WindowContext, Window}, gfx::primitives::DrawRenderer,
};
use ::nt::*;

use crate::{
    constants,
    frc::kinematics::{SwerveDriveKinematics, SwerveModuleState},
    layer::Layer,
};

pub struct Drive {
    chassis_image: Texture,
    wheel_image: Texture,
    last_angles: [f64; 4],
    swerve_kinematics: SwerveDriveKinematics,
    module_states: [SwerveModuleState; 4],
}


fn draw_vector<T>(canvas: &mut Canvas<T>, start: Point, end: Point, color: Color)
where
    T: sdl2::render::RenderTarget,
{
    let dx = (end.x - start.x) as f64;
    let dy = (end.y - start.y) as f64;
    let angle = dy.atan2(dx);
    // let length = (dx * dx + dy * dy).sqrt() as i32;

    let arrow_width = 10;
    let arrow_length = 10;
    let arrow_tip = Point::new(
        end.x + (arrow_length as f64 * angle.cos()) as i32,
        end.y + (arrow_length as f64 * angle.sin()) as i32,
    );
    let arrow_left = Point::new(
        arrow_tip.x - (arrow_width as f64 * (angle + std::f64::consts::PI / 6.0).cos()) as i32,
        arrow_tip.y - (arrow_width as f64 * (angle + std::f64::consts::PI / 6.0).sin()) as i32,
    );
    let arrow_right = Point::new(
        arrow_tip.x - (arrow_width as f64 * (angle - std::f64::consts::PI / 6.0).cos()) as i32,
        arrow_tip.y - (arrow_width as f64 * (angle - std::f64::consts::PI / 6.0).sin()) as i32,
    );
    canvas.set_draw_color(color);
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

impl Layer for Drive {
    fn create(texture_creator: &TextureCreator<WindowContext>) -> Self {
        let chassis_image = texture_creator.load_texture("chassis.png").unwrap();
        let wheel_image = texture_creator.load_texture("wheel.png").unwrap();

        let last_angles = [0.0, 0.0, 0.0, 0.0];

        let swerve_kinematics = SwerveDriveKinematics::new([
            // Front Left
            Vector2::new(
                constants::OFFSET_TO_SWERVE_MODULE_METERS,
                constants::OFFSET_TO_SWERVE_MODULE_METERS,
            ),
            // Back Left
            Vector2::new(
                -constants::OFFSET_TO_SWERVE_MODULE_METERS,
                constants::OFFSET_TO_SWERVE_MODULE_METERS,
            ),
            // Back Right
            Vector2::new(
                -constants::OFFSET_TO_SWERVE_MODULE_METERS,
                -constants::OFFSET_TO_SWERVE_MODULE_METERS,
            ),
            // Front Right
            Vector2::new(
                constants::OFFSET_TO_SWERVE_MODULE_METERS,
                -constants::OFFSET_TO_SWERVE_MODULE_METERS,
            ),
        ]);

        let module_states = [
            SwerveModuleState::default(),
            SwerveModuleState::default(),
            SwerveModuleState::default(),
            SwerveModuleState::default(),
        ];

        Self {
            chassis_image,
            wheel_image,
            last_angles,
            swerve_kinematics,
            module_states,
        }
    }

    fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
        inst: &nt::NetworkTableInstance,
    ) {
        let chassis_src = Rect::new(
            0,
            0,
            self.chassis_image.query().width,
            self.chassis_image.query().height,
        );
        let chassis_dst = Rect::new(100, 0, chassis_src.width(), chassis_src.height());
        canvas
            .copy(&self.chassis_image, chassis_src, chassis_dst)
            .unwrap();
        for module_number in 0..4 {
            // Get the steer angle and velocity for the wheel from NetworkTables
            let mut angle = inst
                .get_entry(format!("/Thunderstorm/Module{}Angle", module_number).as_str())
                .get_value()
                .unwrap()
                .get_double()
                .unwrap_or(0.0);
            let mut velocity_mps = inst
                .get_entry(format!("/Thunderstorm/Module{}Velocity", module_number).as_str())
                .get_value()
                .unwrap()
                .get_double()
                .unwrap_or(0.0);

            self.module_states[module_number] = SwerveModuleState::new(
                velocity_mps,
                &Vector2::new(angle.to_radians().cos(), angle.to_radians().sin()),
            );

            angle = 360.0 - angle;
            velocity_mps = -velocity_mps;

            // SwerveModuleState::optimize()
            let delta = angle - self.last_angles[module_number];

            if delta > 90.0 {
                velocity_mps = -velocity_mps;
                angle += 180.0;
            }
            self.last_angles[module_number] = angle;

            let location = constants::MODULE_PIXEL_LOCATIONS[module_number];
            // Render the wheel.
            let wheel_src = Rect::new(
                0,
                0,
                self.wheel_image.query().width,
                self.wheel_image.query().height,
            );
            let wheel_dst = Rect::new(
                location.0,
                location.1,
                wheel_src.width(),
                wheel_src.height(),
            );
            let center = Point::new(wheel_dst.width() as i32 / 2, wheel_dst.height() as i32 / 2);
            canvas
                .copy_ex(
                    &self.wheel_image,
                    Some(wheel_src),
                    Some(wheel_dst),
                    angle,
                    center,
                    false,
                    false,
                )
                .unwrap();
            if velocity_mps != 0.0_f64 {
                let magnitude = velocity_mps / constants::MAX_SPEED * 100.0;
                let src_point = Point::new(location.0, location.1).offset(20, 46);
                let dst_point = src_point.offset(
                    (magnitude * (angle - 90.0).to_radians().cos()).round() as i32,
                    (magnitude * (angle - 90.0).to_radians().sin()).round() as i32,
                );
                draw_vector(canvas, src_point, dst_point, Color::RGB(0, 0, 0));
                canvas.set_draw_color(Color::RGB(255, 255, 255))
            }
        }

        let chassis_speeds = self
            .swerve_kinematics
            .to_chassis_speeds(&self.module_states);

        let center_robot = Point::new(300, 400);
        let robot_vector_end = center_robot.offset(
            (chassis_speeds.vx_mps / constants::MAX_SPEED * 100.0).round() as i32,
            (chassis_speeds.vy_mps / constants::MAX_SPEED * 100.0).round() as i32,
        );
        draw_vector(canvas, center_robot, robot_vector_end, Color::RGB(0, 0, 0));
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    }
}
