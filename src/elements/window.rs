use super::Element;
use super::w98::*;
use super::DrawContext;
use super::super::theme::*;
use super::super::rectarea::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

pub struct Window
{
	pub name: &'static str,
	pub area: RectArea,
	pub focus: bool,
	pub focusedElement: i32,
	pub elements: std::vec::Vec<Box<dyn super::Element>>
}

impl Element for Window
{
	fn intersection(&self, ra: RectArea) -> bool
	{
		return self.area.intersects(ra);
	}

	fn offsetPosition(&mut self, offset: XY)
	{

		self.area.pos.x += offset.x;
		self.area.pos.y += offset.y;
		for e in &mut self.elements
		{
			e.as_mut().offsetPosition(offset);
		}
	}

	fn handleInput(&mut self, event: &Event)
	{
		match *event
		{
			Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } =>
			{
				let mra = RectArea { pos: XY { x, y }, siz: XY { x: 1, y: 1 }};
				let mut found = false;
				for ei in 0..self.elements.len()
				{
					if self.elements[ei].intersection(mra)
					{
						self.focusedElement = ei as i32;
						found = true;
					}
				}
				if !found
				{
					self.focusedElement = -1;
				}
			},
			_ => ()
		}

		if self.focusedElement >= 0
		{
			self.elements[self.focusedElement as usize].handleInput(event);
		}
		//for e in &mut self.elements
		//{
		//	e.handleInput(&event);
		//}
	}

	fn draw(&mut self, draw_context: &mut DrawContext, theme: &w98Theme)
	{
		let mut canvas = &mut draw_context.canvas;

		// This is just to help visually identify window-drawing bugs. Red should not be visible
		canvas.set_draw_color(Color::RED);
		canvas.fill_rect(Rect::new(self.area.pos.x, self.area.pos.y, self.area.siz.x as u32, self.area.siz.y as u32)).expect("windowBackground");
	
		drawW983dBox(&mut canvas, &theme.window.border, theme.window.backgroundColor, &self.area);

		canvas.set_draw_color(if self.focus { theme.window.titleBar.color } else { theme.window.titleBar.inactiveColor });
		let titleRect = Rect::new(self.area.pos.x + theme.window.border.left.outer.width as i32 + theme.window.border.left.inner.width as i32 + theme.window.titleBorder.left.width as i32, self.area.pos.y + theme.window.border.top.outer.width as i32 + theme.window.border.top.inner.width as i32 + theme.window.titleBorder.top.width as i32, self.area.siz.x as u32 - theme.window.border.left.outer.width - theme.window.border.left.inner.width - theme.window.border.right.inner.width - theme.window.border.right.outer.width - theme.window.titleBorder.left.width - theme.window.titleBorder.right.width, theme.window.titleBar.width);
		canvas.fill_rect(titleRect).expect("title");

		let hDiff: i32 = (theme.window.titleBar.width - theme.window.titleFont.points as u32) as i32;
		writeLeftAligned(&mut canvas, &draw_context.texture_creator, &draw_context.font, &theme.window.titleFont.color, XY {x: titleRect.x() + hDiff, y: titleRect.y() + (hDiff / 2)}, (self.area.siz.x - hDiff*2) as u32, self.name);

		for e in &mut self.elements
		{
			e.draw(draw_context, theme);
		}
	}
}