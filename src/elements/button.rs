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
	area: RectArea,
	text: String,
	targeted: bool,
	pressed: bool
}

impl Button
{
	pub fn new(text: String) -> Self
	{
		return Button { area: ORIGIN_ZERO, text, targeted: false, pressed: false };
	}
}

impl Element for Button
{
	fn position(&mut self, area: &RectArea)
	{
		self.area = area.clone();
	}

	fn target(&mut self, on: bool)
	{
		self.targeted = on;
	}

	fn handleInput(&mut self, event: &Event)
	{
		match *event
		{
			Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => 
			{
				if self.area.intersects2(x, y, 1, 1)
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
		let hDiff2: i32 = self.area.h as i32 - theme.font.points as i32;
		writeLeftAligned(&mut draw_context.canvas, &draw_context.texture_creator, &draw_context.font, &Color::BLACK, XY {x: self.area.x + hDiff2, y: self.area.y - (theme.window.border.top.inner.width + theme.window.border.top.outer.width) as i32 + (hDiff2 / 2)}, self.area.w - hDiff2 as u32*2, &self.text);
	}
}