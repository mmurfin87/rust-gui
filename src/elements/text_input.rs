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
	area: RectArea,
	text: String,
	cursorIndex: usize,
	frames: u32,
	targeted: bool
}

impl TextInput
{
	pub fn new() -> Self
	{
		return TextInput { area: ORIGIN_ZERO, text: String::new(), cursorIndex: 0, frames: 0, targeted: false };
	}

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
					Keycode::Delete =>
					{
						if self.cursorIndex < self.text.len()
						{
							self.text = self.text[0..self.cursorIndex].to_owned() + &self.text[self.cursorIndex+1..];
						}
					}
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
		let innerArea = innerRectArea(&theme.window.border, &self.area);
		let hDiff2: u32 = innerArea.h - theme.font.points as u32;
		let maxWidth = (innerArea.w - hDiff2*2) as u32;
		if !self.text.is_empty()
		{
			writeLeftAligned(&mut draw_context.canvas, &draw_context.texture_creator, &draw_context.font, &Color::BLACK, XY { x: innerArea.x, y: innerArea.y}.offset(hDiff2 as i32, hDiff2 as i32 / 2), maxWidth, &self.text);
		}
		if self.targeted && self.frames < 21
		{
			let textWidth = draw_context.font.size_of(&self.text[..self.cursorIndex]).expect("Text Width");
			draw_context.canvas.set_draw_color(Color::BLACK);
			draw_context.canvas.draw_rect(innerArea.adjusted((hDiff2 + textWidth.0).min(maxWidth) as i32, hDiff2 as i32 / 2, -(innerArea.w as i32 - 1), -(hDiff2 as i32)).to_rect()).expect("cursor");
		}
		else if self.targeted && self.frames > 40
		{
			self.frames = 0;
		}
		
		self.frames += 1;
	}
}