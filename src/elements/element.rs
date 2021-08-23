use super::super::rectarea::RectArea;
use super::super::theme::w98Theme;
use super::DrawContext;
use sdl2::event::Event;

pub trait Element
{
	fn position(&mut self, area: &RectArea);
	fn target(&mut self, on: bool);
	fn handleInput(&mut self, event: &Event);
	fn draw(&mut self, draw_context: &mut DrawContext, theme: &w98Theme);
}