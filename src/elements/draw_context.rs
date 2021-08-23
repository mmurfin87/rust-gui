use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct DrawContext<'ttf_context>
{
	pub canvas: Canvas<Window>,
	pub font: Font<'ttf_context, 'static>,
	pub texture_creator: TextureCreator<WindowContext>,
	lineSpacing: u32
}

impl<'ttf_context> DrawContext<'ttf_context>
{
	pub fn new(canvas: Canvas<sdl2::video::Window>, font: Font<'ttf_context, 'static>) -> Self
	{		
		let lineSpacing = (font.recommended_line_spacing() + font.descent()) as u32;
		let texture_creator = canvas.texture_creator();
		
		DrawContext { canvas, font, texture_creator, lineSpacing }
	}

	pub fn emCoord(&self, em: u16) -> u32
	{
		self.lineSpacing * em as u32
	}

	pub fn textWidth(&self, text: &str, extraEms: u16) -> u32
	{
		self.font.size_of(text).expect("textWidth").0 + self.lineSpacing * extraEms as u32
	}
}