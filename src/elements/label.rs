use super::Element;
use super::DrawContext;
use super::w98::*;
use super::super::theme::*;
use super::super::rectarea::*;
use sdl2::pixels::Color;
use sdl2::event::Event;

pub struct Label
{
	area: RectArea,
	text: String
}

impl Label
{
	pub fn new(text: String) -> Self
	{
		return Label { area: ORIGIN_ZERO, text };
	}
}

impl Element for Label
{
	fn position(&mut self, area: &RectArea)
	{
		self.area = area.clone();
	}

	fn target(&mut self, on: bool)
	{
		
	}

	fn handleInput(&mut self, event: &Event)
	{
		
	}

	fn draw(&mut self, draw_context: &mut DrawContext, theme: &w98Theme)
	{
		if !self.text.is_empty()
		{
			let hDiff2: i32 = self.area.h as i32 - theme.font.points as i32;
			writeLeftAligned(&mut draw_context.canvas, &draw_context.texture_creator, &draw_context.font, &Color::BLACK, XY {x: self.area.x /*+ hDiff2*/, y: self.area.y - (theme.window.border.top.inner.width + theme.window.border.top.outer.width) as i32 + (hDiff2 / 2)}, self.area.w - hDiff2 as u32*2, &self.text);
		}
	}
}