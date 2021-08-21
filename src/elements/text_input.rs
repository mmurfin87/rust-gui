use super::Element;
use super::DrawContext;
use super::w98::*;
use super::super::theme::*;
use super::super::rectarea::*;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

pub struct TextInput
{
	pub area: RectArea,
	pub text: String,
	pub cursorIndex: u32,
	pub frames: u32
}

impl Element for TextInput
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
		match event
		{
			Event::KeyDown { keycode: Some(kc), .. } =>
			{
				match kc
				{
					Keycode::Backspace =>
					{
						if !self.text.is_empty() 
						{
							self.cursorIndex -= 1;
							self.text = self.text[0..self.text.len()-1].to_owned();
						}
					},
					Keycode::Return =>
					{
						println!("Text: {}", self.text);
						self.cursorIndex = 0;
						self.text.clear();
					},
					_ => ()
				}
			},
			Event::TextInput { text, .. } =>
			{
				self.cursorIndex += 1;
				self.text += &text;
			},
			_ => ()
		}
	}

	fn draw(&mut self, draw_context: &mut DrawContext, theme: &w98Theme)
	{
		drawW983dBox(&mut draw_context.canvas, &theme.window.border.invert(), Color::WHITE, &self.area);
		let hDiff2: i32 = self.area.siz.y - theme.font.points as i32;
		let borderWidth: u32 = theme.window.titleBorder.top.width + theme.window.titleBorder.bottom.width;
		let origin: XY = XY {x: self.area.pos.x + hDiff2, y: self.area.pos.y - borderWidth as i32 + (hDiff2 / 2)};
		let textWidth = if !self.text.is_empty()
		{
			writeLeftAligned(&mut draw_context.canvas, &draw_context.texture_creator, &draw_context.font, &Color::BLACK, &origin, (self.area.siz.x - hDiff2*2) as u32, &self.text)
		}
		else 
		{ 0 };
		if self.frames < 21
		{
			draw_context.canvas.set_draw_color(Color::BLACK);
			draw_context.canvas.draw_rect(sdl2::rect::Rect::new(origin.x + textWidth as i32, origin.y, 2, self.area.siz.y as u32 - borderWidth)).expect("cursor");
		}
		else if self.frames > 40
		{
			self.frames = 0;
		}
		
		self.frames += 1;
	}
}