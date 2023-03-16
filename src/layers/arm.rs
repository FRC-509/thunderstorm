use crate::layer::Layer;

pub struct Arm {}

impl Layer for Arm {
    fn create(texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        todo!()
    }

    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        inst: &nt::nt::NetworkTableInstance,
    ) {
        todo!()
    }
}
