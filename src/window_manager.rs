use super::rectarea::*;
use super::theme::*;
use super::elements::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub struct WindowManager<'wm>
{
	pub quit: bool,
	theme: &'wm w98Theme,
	draw_context: DrawContext<'wm>,
	windows: std::vec::Vec<Window>,
	useClassicTheme: bool,
	capturedWindow: bool
}

impl<'wm> WindowManager<'wm>
{
	pub fn new(draw_context: DrawContext<'wm>) -> Self
	{
		WindowManager { quit: false, draw_context, windows: std::vec::Vec::new(), theme: &CLASSIC_W98_THEME, useClassicTheme: true, capturedWindow: false}
	}

	pub fn addWindow(&mut self, mut window: Window, area: RectArea) -> &mut Window
	{
		window.position(&area);
		window.titleArea = titleRectArea(&area, self.theme);
		self.windows.push(window);
		self.windows.last_mut().unwrap()
	}

	pub fn emCoord(&self, em: u16) -> u32
	{
		self.draw_context.emCoord(em)
	}

	pub fn textWidth(&self, text: &String, lPadEms: u16, rPadEms: u16) -> u32
	{
		self.draw_context.textWidth(text, lPadEms, rPadEms)
	}

	pub fn verticalBorderSize(&self) -> u32
	{
		self.theme.window.border.top.outer.width + self.theme.window.border.top.inner.width + self.theme.window.border.bottom.inner.width + self.theme.window.border.bottom.outer.width
	}

	pub fn handleInput(&mut self, event: &Event)
	{
		match *event
		{
			Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => self.quit = true,
			Event::MouseButtonDown { mouse_btn, x, y, .. } =>
			{
				let logButtonDown = |button: &str| println!("Mouse Down: {} at ({}, {})", button, x, y);
				match mouse_btn
				{
					MouseButton::Left => 
					{
						let mra = RectArea::new(x, y, 1, 1);
						let mut activatedWindow = false;
						for wi in 0..self.windows.len()
						{
							if self.windows[wi].area.intersects(mra)
							{
								activatedWindow = true;
								if wi > 0
								{
									self.windows.swap(0, wi);
									self.windows[wi].focus = false;
									self.windows[wi].target(false);
									self.windows[0].focus = true;
									self.windows[0].target(true);
								}
								else if !self.windows[0].focus
								{
									self.windows[0].focus = true;
									self.windows[0].target(true);
								}
								
								//let titlera = titleRectArea(&self.windows[0].area, &self.theme);
								if self.windows[0].titleArea.intersects(mra)//titleBarCollision(x, y, &titlera)
								{
									self.capturedWindow = true;
									break;
								}
							}
						}
						if !activatedWindow
						{
							self.windows[0].target(false);
							self.windows[0].focus = false;
						}
						logButtonDown("Left")
					},
					MouseButton::Middle => logButtonDown("Middle"),
					MouseButton::Right =>
					{
						self.useClassicTheme = !self.useClassicTheme;
						self.theme = if self.useClassicTheme { &CLASSIC_W98_THEME } else { &MYSTERY_W98_THEME };
						logButtonDown("Right");
					},
					MouseButton::X1 => logButtonDown("X1"),
					MouseButton::X2 => logButtonDown("X2"),
					_ => logButtonDown("Unknown")
				};
			},
			Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => self.capturedWindow = false,
			Event::MouseMotion { mousestate, xrel, yrel, .. } =>
			{
				let mut buttons = std::vec::Vec::new();
				if mousestate.left() { buttons.push("Left"); }
				if mousestate.middle() { buttons.push("Middle"); }
				if mousestate.right() { buttons.push("Right"); }
				if mousestate.x1() { buttons.push("X1"); }
				if mousestate.x2() { buttons.push("X2"); }

				if self.capturedWindow
				{
					let newPosition = self.windows[0].area.adjusted(xrel, yrel, 0, 0);
					self.windows[0].position(&newPosition);
					//self.windows[0].area = self.windows[0].area.adjusted(xrel, yrel, 0, 0);
				}
			},
			_ => {}
		}
		
		for window in &mut self.windows
		{
			window.handleInput(event);
		}
	}

	pub fn draw(&mut self)
	{
		self.draw_context.canvas.set_draw_color(self.theme.backgroundColor);
		self.draw_context.canvas.clear();

		for wi in (0..self.windows.len()).rev()
		{
			// This is just to help visually identify window-drawing bugs. Red should not be visible
			self.draw_context.canvas.set_draw_color(Color::RED);
			self.draw_context.canvas.fill_rect(Rect::new(self.windows[wi].area.x, self.windows[wi].area.y, self.windows[wi].area.w, self.windows[wi].area.h)).expect("windowBackground");
	
			drawW983dBox(&mut self.draw_context.canvas, &self.theme.window.border, self.theme.window.backgroundColor, &self.windows[wi].area);

			self.draw_context.canvas.set_draw_color(if self.windows[wi].focus { self.theme.window.titleBar.color } else { self.theme.window.titleBar.inactiveColor });
			let titleRect = Rect::new(self.windows[wi].area.x + self.theme.window.border.left.outer.width as i32 + self.theme.window.border.left.inner.width as i32 + self.theme.window.titleBorder.left.width as i32, (self.windows[wi].area.y as u32 + self.theme.window.border.top.outer.width + self.theme.window.border.top.inner.width + self.theme.window.titleBorder.top.width) as i32, self.windows[wi].area.w as u32 - self.theme.window.border.left.outer.width - self.theme.window.border.left.inner.width - self.theme.window.border.right.inner.width - self.theme.window.border.right.outer.width - self.theme.window.titleBorder.left.width - self.theme.window.titleBorder.right.width, self.theme.window.titleBar.width);
			self.draw_context.canvas.fill_rect(titleRect).expect("title");

			let hDiff: u32 = self.theme.window.titleBar.width - self.theme.window.titleFont.points as u32;
			writeLeftAligned(&mut self.draw_context.canvas, &self.draw_context.texture_creator, &self.draw_context.font, &self.theme.window.titleFont.color, XY {x: titleRect.x() + hDiff as i32, y: titleRect.y() + (hDiff as i32 / 2)}, self.windows[wi].area.w - hDiff*2, self.windows[wi].name);

			self.windows[wi].draw(&mut self.draw_context, &self.theme);
		}

		self.draw_context.canvas.present();
	}
}
fn titleRectArea(w: &RectArea, theme: &w98Theme) -> RectArea
{
	return RectArea
	{
		x: w.x + (theme.window.border.left.outer.width + theme.window.border.left.inner.width + theme.window.titleBorder.left.width) as i32,
		y: w.y + (theme.window.border.top.outer.width + theme.window.border.top.inner.width + theme.window.titleBorder.top.width) as i32,
		w: w.w - theme.window.border.left.outer.width - theme.window.border.left.inner.width - theme.window.border.right.inner.width - theme.window.border.right.outer.width - theme.window.titleBorder.left.width - theme.window.titleBorder.right.width, 
		h: theme.window.titleBar.width
	};
}