use crate::{constants, layer::Layer};
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

pub struct Arm {
    origin: Point,
    robot_image: Texture,
    claw_image: Texture,
    arm_image: Texture,
    robot_src: Rect,
    claw_src: Rect,
    arm_src: Rect,
}

impl Layer for Arm {
    fn create(texture_creator: &TextureCreator<WindowContext>, origin: Point) -> Self {
        let robot_image = texture_creator.load_texture("robot.png").unwrap();
        let claw_image = texture_creator.load_texture("claw.png").unwrap();
        let arm_image = texture_creator.load_texture("arm.png").unwrap();
        let robot_src = Rect::new(0, 0, robot_image.query().width, robot_image.query().height);
        let claw_src = Rect::new(0, 0, claw_image.query().width, claw_image.query().height);
        let arm_src = Rect::new(0, 0, arm_image.query().width, arm_image.query().height);
        Self {
            origin,
            robot_image,
            claw_image,
            arm_image,
            robot_src,
            claw_src,
            arm_src,
        }
    }

    fn render(&mut self, canvas: &mut Canvas<Window>, inst: &nt::NetworkTableInstance) {
        // Grab pivot and extension from NetworkTables.
        let pivot = inst
            .get_entry("/Thunderstorm/ArmPivot")
            .get_value()
            .unwrap()
            .get_double()
            .unwrap_or(0.0);
        let extension_sensor_units = inst
            .get_entry("/Thunderstorm/ArmExtension")
            .get_value()
            .unwrap()
            .get_double()
            .unwrap_or(0.0);

        // Grab rects and center axes for rendering the arm and end effector.
        let mut arm_dst = Rect::new(220, 32, self.arm_src.width(), self.arm_src.height());
        arm_dst.offset(self.origin.x(), self.origin.y());
        let cor_arm = Point::new(409 + self.origin.x(), 28 + self.origin.y()) - arm_dst.top_left();
        // Convert the extension from sensor units into pixels.
        let extension_pixels =
            (extension_sensor_units / constants::MAX_EXTENSION * 158.0).round() as i32;
        let mut claw_dst = Rect::new(
            158 - extension_pixels,
            19,
            self.claw_src.width(),
            self.claw_src.height(),
        );
        claw_dst.offset(self.origin.x(), self.origin.y());
        let cor_claw = Point::new(409 + self.origin.x(), 28 + self.origin.y()) - claw_dst.top_left();

        let mut robot_dst = Rect::new(263, 0, self.robot_src.width(), self.robot_src.height());
        robot_dst.offset(self.origin.x(), self.origin.y());

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
    }
}
