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
	pub cursorIndex: usize,
	pub frames: u32
}

impl TextInput
{
	fn shiftCursorLeft(&mut self)
	{
		if self.cursorIndex > 0
		{
			self.cursorIndex -= 1;
		}
	}

	fn shiftCursorRight(&mut self)
	{
		if self.cursorIndex < self.text.len()
		{
			self.cursorIndex += 1;
		}
	}

	fn setCursor(&mut self, x: usize)
	{
		self.cursorIndex = if x == 0 { 0 } else if x > self.text.len() { self.text.len() } else { x };
	}
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
					Keycode::Left => self.shiftCursorLeft(),
					Keycode::Right => self.shiftCursorRight(),
					Keycode::Home => self.setCursor(0),
					Keycode::End => self.setCursor(self.text.len()),
					_ => ()
				}
			},
			Event::TextInput { text, .. } =>
			{
				self.text = self.text[..self.cursorIndex].to_owned() + text + &self.text[self.cursorIndex..].to_owned();
				self.cursorIndex += 1;
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
		let maxWidth = (self.area.siz.x - hDiff2*2) as u32;
		if !self.text.is_empty()
		{
			writeLeftAligned(&mut draw_context.canvas, &draw_context.texture_creator, &draw_context.font, &Color::BLACK, origin, maxWidth, &self.text);
		}
		if self.frames < 21
		{
			let textWidth = draw_context.font.size_of(&self.text[..self.cursorIndex]).expect("Text Width");
			draw_context.canvas.set_draw_color(Color::BLACK);
			draw_context.canvas.draw_rect(sdl2::rect::Rect::new(origin.x + (textWidth.0 as i32).min(maxWidth as i32), origin.y, 2, self.area.siz.y as u32 - borderWidth)).expect("cursor");
		}
		else if self.frames > 40
		{
			self.frames = 0;
		}
		
		self.frames += 1;
	}
}