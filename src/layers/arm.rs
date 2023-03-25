use crate::{constants, layer::Layer, mulr};
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

pub struct Arm {
    origin: Point,
    scale: f64,
    robot_image: Texture,
    claw_image: Texture,
    arm_image: Texture,
    red_bumper_image: Texture,
    blue_bumper_image: Texture,
    robot_src: Rect,
    claw_src: Rect,
    arm_src: Rect,
    bumper_src: Rect,
}

impl Layer for Arm {
    fn create(texture_creator: &TextureCreator<WindowContext>, origin: Point, scale: f64) -> Self {
        let robot_image = texture_creator.load_texture("robot.png").unwrap();
        let claw_image = texture_creator.load_texture("claw.png").unwrap();
        let arm_image = texture_creator.load_texture("arm.png").unwrap();
        let red_bumper_image = texture_creator.load_texture("red_bumper.png").unwrap();
        let blue_bumper_image = texture_creator.load_texture("blue_bumper.png").unwrap();
        let robot_src = Rect::new(0, 0, robot_image.query().width, robot_image.query().height);
        let claw_src = Rect::new(0, 0, claw_image.query().width, claw_image.query().height);
        let arm_src = Rect::new(0, 0, arm_image.query().width, arm_image.query().height);
        let bumper_src = Rect::new(
            0,
            0,
            red_bumper_image.query().width,
            red_bumper_image.query().height,
        );
        Self {
            origin: Point::new(mulr!(origin.x(), scale), mulr!(origin.y(), scale)),
            scale,
            robot_image,
            claw_image,
            arm_image,
            red_bumper_image,
            blue_bumper_image,
            robot_src,
            claw_src,
            arm_src,
            bumper_src,
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>, inst: &nt::NetworkTableInstance) {
        // Grab pivot and extension from NetworkTables.
        let pivot = inst
            .get_entry("/Thunderstorm/ArmPivot")
            .get_value()
            .unwrap()
            .get_double()
            .unwrap_or(0.0)
            - 90.0;
        let extension_sensor_units = inst
            .get_entry("/Thunderstorm/ArmExtension")
            .get_value()
            .unwrap()
            .get_double()
            .unwrap_or(0.0);
        let is_red_alliance = inst
            .get_entry("/FMSInfo/IsRedAlliance")
            .get_value()
            .unwrap()
            .get_boolean()
            .unwrap_or(true);

        // Grab rects and center axes for rendering the arm and end effector.
        let mut arm_dst = Rect::new(
            mulr!(220, self.scale),
            mulr!(32, self.scale),
            mulr!(self.arm_src.width(), self.scale) as u32,
            mulr!(self.arm_src.height(), self.scale) as u32,
        );
        arm_dst.offset(self.origin.x(), self.origin.y());
        let cor_arm = Point::new(
            mulr!(409, self.scale) + self.origin.x(),
            mulr!(28, self.scale) + self.origin.y(),
        ) - arm_dst.top_left();
        // Convert the extension from sensor units into pixels.
        let extension_pixels =
            (extension_sensor_units / constants::MAX_EXTENSION * 158.0).round() as i32;
        let mut claw_dst = Rect::new(
            mulr!(158 - extension_pixels, self.scale),
            mulr!(19, self.scale),
            mulr!(self.claw_src.width(), self.scale) as u32,
            mulr!(self.claw_src.height(), self.scale) as u32,
        );
        claw_dst.offset(self.origin.x(), self.origin.y());
        let cor_claw = Point::new(
            mulr!(409, self.scale) + self.origin.x(),
            mulr!(28, self.scale) + self.origin.y(),
        ) - claw_dst.top_left();

        let mut robot_dst = Rect::new(
            mulr!(263, self.scale),
            mulr!(0, self.scale),
            mulr!(self.robot_src.width(), self.scale) as u32,
            mulr!(self.robot_src.height(), self.scale) as u32,
        );
        robot_dst.offset(self.origin.x(), self.origin.y());

        let mut bumper_dst = Rect::new(
            mulr!(263, self.scale),
            mulr!(315, self.scale),
            mulr!(self.bumper_src.width(), self.scale) as u32,
            mulr!(self.bumper_src.height(), self.scale) as u32,
        );
        bumper_dst.offset(self.origin.x(), self.origin.y());

        // Render the end effector.
        canvas
            .copy_ex(
                &self.claw_image,
                Some(self.claw_src),
                Some(claw_dst),
                pivot,
                cor_claw,
                false,
                false,
            )
            .unwrap();
        // Render the arm.
        canvas
            .copy_ex(
                &self.arm_image,
                Some(self.arm_src),
                Some(arm_dst),
                pivot,
                cor_arm,
                false,
                false,
            )
            .unwrap();
        // Render the robot texture.
        canvas
            .copy(&self.robot_image, self.robot_src, robot_dst)
            .unwrap();
        // Render the bumper texture.
        if is_red_alliance {
            canvas
                .copy(&self.red_bumper_image, self.bumper_src, bumper_dst)
                .unwrap();
        } else {
            canvas
                .copy(&self.blue_bumper_image, self.bumper_src, bumper_dst)
                .unwrap();
        }
    }
}
