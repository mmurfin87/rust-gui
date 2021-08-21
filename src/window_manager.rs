use super::rectarea::*;
use super::theme::*;
use super::elements::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

const fn titleBarCollision(x: i32, y: i32, titleBar: &RectArea) -> bool
{
	return x >= titleBar.pos.x && x <= titleBar.pos.x + titleBar.siz.x && y >= titleBar.pos.y && y <= titleBar.pos.y + titleBar.siz.y;
}

const fn titleRectArea(w: &RectArea, theme: &w98Theme) -> RectArea
{
	return RectArea
	{
		pos: XY
		{
			x: w.pos.x + theme.window.border.left.outer.width as i32 + theme.window.border.left.inner.width as i32 + theme.window.titleBorder.left.width as i32, 
			y: w.pos.y + theme.window.border.top.outer.width as i32 + theme.window.border.top.inner.width as i32 + theme.window.titleBorder.top.width as i32
		}, 
		siz: XY
		{
			x: w.siz.x - theme.window.border.left.outer.width as i32 - theme.window.border.left.inner.width as i32 - theme.window.border.right.inner.width as i32 - theme.window.border.right.outer.width as i32 - theme.window.titleBorder.left.width as i32 - theme.window.titleBorder.right.width as i32, 
			y: theme.window.titleBar.width as i32
		}
	};
}

pub struct WindowManager
{
	pub quit: bool,
	pub windows: std::vec::Vec<Window>,
	pub theme: &'static w98Theme,
	pub useClassicTheme: bool,
	pub capturedWindow: bool
}

impl WindowManager
{
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
						let mra = RectArea { pos: XY { x, y }, siz: XY { x: 1, y: 1 }};
						for wi in 0..self.windows.len()
						{
							if self.windows[wi].area.intersects(mra)
							{
								if wi > 0
								{
									self.windows.swap(0, wi);
									self.windows[0].focus = true;
									self.windows[wi].focus = false;
								}
								
								let titlera = titleRectArea(&self.windows[0].area, &self.theme);
								if titleBarCollision(x, y, &titlera)
								{
									self.capturedWindow = true;
									break;
								}
							}
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
					self.windows[0].offsetPosition(XY { x: xrel, y: yrel });
				}
			},
			_ => {}
		}
		
		for window in &mut self.windows
		{
			window.handleInput(event);
		}
	}

	pub fn draw(&mut self, mut draw_context: &mut DrawContext)
	{
		draw_context.canvas.set_draw_color(self.theme.backgroundColor);
		draw_context.canvas.clear();

		for wi in (0..self.windows.len()).rev()
		{
			self.windows[wi].draw(&mut draw_context, &self.theme);
		}

		draw_context.canvas.present();
	}
}