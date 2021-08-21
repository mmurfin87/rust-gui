use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct DrawContext<'ttf_context>
{
	pub canvas: Canvas<Window>,
	pub font: Font<'ttf_context, 'static>,
	pub texture_creator: TextureCreator<WindowContext>
}