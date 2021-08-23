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
	pub titleArea: RectArea,
	pub focus: bool,
	pub focusedElement: i32,
	spacing: u32,
	pub elements: std::vec::Vec<ElementData>
}

impl Window
{
	pub fn new(name: &'static str, spacing: u32) -> Self
	{
		return Window
		{
			name,
			area: ORIGIN_ZERO,
			titleArea: ORIGIN_ZERO,
			focus: false,
			focusedElement: -1,
			spacing,
			elements: std::vec::Vec::new()
		};
	}

	pub fn addElement(&mut self, width: u32, height: u32, element: Box<dyn Element>) -> &mut Window
	{
		let yOffset: i32 = self.elements.last().map(|e| e.area.y + e.area.h as i32).unwrap_or(0);
		let area = RectArea::new(self.spacing as i32, yOffset + self.spacing as i32, width, height);
		println!("Added Element with yOffset {} at {:?}", yOffset, area);
		let mut data = ElementData{ area, element};
		data.element.position(&area.adjusted(self.titleArea.x, self.titleArea.y + self.titleArea.h as i32, 0, 0));
		self.elements.push(data);
		self
	}

	fn repositionElement(titleArea: &RectArea, element: &mut ElementData)
	{
		element.element.position(&element.area.adjusted(titleArea.x, titleArea.y + titleArea.h as i32, 0, 0));
	}
}

pub struct ElementData
{
	area: RectArea,	// x,y stores the offset from window origin, not the absolute position
	element: Box<dyn Element>
}

impl Element for Window
{
	fn position(&mut self, area: &RectArea)
	{
		let xrel = area.x - self.area.x;
		let yrel = area.y - self.area.y;
		let wrel = area.w as i32 - self.area.w as i32;
		let hrel = area.h as i32 - self.area.h as i32;
		self.area = area.clone();
		self.titleArea = self.titleArea.adjusted(xrel, yrel, wrel, hrel);
		for e in &mut self.elements
		{
			e.element.position(&e.area.adjusted(self.titleArea.x, self.titleArea.y + self.titleArea.h as i32, 0, 0));
			//let pos = self.elementAbsolutePosition(&e.area);
			//e.element.position(&pos);
			//Window::repositionElement(&self.titleArea, e);//e.element.position(&e.area.adjusted(self.titleArea.x, self.titleArea.y + self.titleArea.h as i32, 0, 0));
		}
	}

	fn target(&mut self, on: bool)
	{
		
	}

	fn handleInput(&mut self, event: &Event)
	{
		match *event
		{
			Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } =>
			{
				let mut found = false;
				for ei in 0..self.elements.len()
				{
					if self.elements[ei].area.intersects2(x - self.titleArea.x, y - self.titleArea.y - self.titleArea.h as i32, 1, 1)
					{
						if self.focusedElement != -1
						{
							self.elements[self.focusedElement as usize].element.target(false);
						}
						self.focusedElement = ei as i32;
						found = true;
						self.elements[ei].element.target(true);
					}
				}
				if !found
				{
					if self.focusedElement != -1
					{
						self.elements[self.focusedElement as usize].element.target(false);
					}
					self.focusedElement = -1;
				}
			},
			_ => ()
		}

		if self.focusedElement >= 0
		{
			self.elements[self.focusedElement as usize].element.handleInput(event);
		}
	}

	fn draw(&mut self, draw_context: &mut DrawContext, theme: &w98Theme)
	{
		let mut canvas = &mut draw_context.canvas;

		// This is just to help visually identify window-drawing bugs. Red should not be visible
		canvas.set_draw_color(Color::RED);
		canvas.fill_rect(Rect::new(self.area.x, self.area.y, self.area.w, self.area.h)).expect("windowBackground");
	
		drawW983dBox(&mut canvas, &theme.window.border, theme.window.backgroundColor, &self.area);

		canvas.set_draw_color(if self.focus { theme.window.titleBar.color } else { theme.window.titleBar.inactiveColor });
		let titleRect = Rect::new(self.area.x + theme.window.border.left.outer.width as i32 + theme.window.border.left.inner.width as i32 + theme.window.titleBorder.left.width as i32, (self.area.y as u32 + theme.window.border.top.outer.width + theme.window.border.top.inner.width + theme.window.titleBorder.top.width) as i32, self.area.w as u32 - theme.window.border.left.outer.width - theme.window.border.left.inner.width - theme.window.border.right.inner.width - theme.window.border.right.outer.width - theme.window.titleBorder.left.width - theme.window.titleBorder.right.width, theme.window.titleBar.width);
		canvas.fill_rect(titleRect).expect("title");

		let hDiff: u32 = theme.window.titleBar.width - theme.window.titleFont.points as u32;
		writeLeftAligned(&mut canvas, &draw_context.texture_creator, &draw_context.font, &theme.window.titleFont.color, XY {x: titleRect.x() + hDiff as i32, y: titleRect.y() + (hDiff as i32 / 2)}, self.area.w - hDiff*2, self.name);

		for e in &mut self.elements
		{
			e.element.draw(draw_context, theme);
		}
	}
}