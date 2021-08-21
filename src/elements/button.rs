use super::Element;
use super::DrawContext;
use super::w98::*;
use super::super::theme::*;
use super::super::rectarea::*;
use sdl2::pixels::Color;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;

pub struct Button
{
	pub area: RectArea,
	pub text: String,
	pub pressed: bool
}

impl Element for Button
{
	fn intersection(&self, ra: RectArea) -> bool
	{
		return self.area.intersects(ra);
	}

	fn offsetPosition(&mut self, offset: XY)
	{
		self.area.pos.x += offset.x;
		self.area.pos.y += offset.y;
	}

	fn handleInput(&mut self, event: &Event)
	{
		match *event
		{
			Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => 
			{
				if self.intersection(RectArea { pos: XY { x: x, y: y }, siz: XY { x: 1, y: 1}})
				{
					self.pressed = true
				}
			},
			Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } =>
			{
				self.pressed = false;
			},
			_ => ()
		}
	}

	fn draw(&mut self, draw_context: &mut DrawContext, theme: &w98Theme)
	{
		if self.pressed
		{
			let invertedTheme = theme.window.border.invert();
			drawW983dBox(&mut draw_context.canvas, &invertedTheme, theme.window.backgroundColor, &self.area);
		}
		else
		{
			drawW983dBox(&mut draw_context.canvas, &theme.window.border, theme.window.backgroundColor, &self.area);
		}
		let hDiff2: i32 = self.area.siz.y - theme.font.points as i32;
		writeLeftAligned(&mut draw_context.canvas, &draw_context.texture_creator, &draw_context.font, &Color::BLACK, &XY {x: self.area.pos.x + hDiff2, y: self.area.pos.y - (theme.window.border.top.inner.width + theme.window.border.top.outer.width) as i32 + (hDiff2 / 2)}, (self.area.siz.x - hDiff2*2) as u32, &self.text);
	}
}