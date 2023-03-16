use ::nt::*;
use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

pub trait Layer {
    fn create(texture_creator: &TextureCreator<WindowContext>) -> Self;
    fn render(&mut self, canvas: &mut Canvas<Window>, inst: &nt::NetworkTableInstance);
}
