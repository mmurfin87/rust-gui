use super::super::theme::*;
use super::super::rectarea::*;
use super::DrawContext;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub trait Element
{
	fn intersection(&self, ra: RectArea) -> bool;
	fn offsetPosition(&mut self, offset: XY);
	fn handleInput(&mut self, event: &Event);
	fn draw(&mut self, draw_context: &mut DrawContext, theme: &w98Theme);
}