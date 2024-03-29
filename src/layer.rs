use sdl2::{
    rect::Point,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

pub trait Layer {
    fn create(texture_creator: &TextureCreator<WindowContext>, origin: Point, scale: f64) -> Self;
    fn render(&mut self, canvas: &mut Canvas<Window>, inst: &nt::NetworkTableInstance);
}
