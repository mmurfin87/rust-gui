#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate sdl2;

mod elements;
mod rectarea;
mod theme;
mod window_manager;

use rectarea::*;
use theme::*;
use elements::*;
use window_manager::WindowManager;
use std::time::Duration;
use sdl2::render::Canvas;

fn main() {
    println!("Hello, world!");

	
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let ttf_context = sdl2::ttf::init().unwrap();

	let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
		.position_centered()
		.build()
		.unwrap();
	let canvas: Canvas<sdl2::video::Window> = window.into_canvas().build().expect("Could not make a canvas");
	
	let theme: &w98Theme = &CLASSIC_W98_THEME;
	let mut font = ttf_context.load_font(theme.font.path, theme.font.points).unwrap();
	font.set_style(sdl2::ttf::FontStyle::NORMAL);
	
	let mut windowManager = WindowManager::new(DrawContext::new(canvas, font));
	
	let elementSpacing = theme.font.points as u32 + 4;
	let verticalBorderSize = windowManager.verticalBorderSize();
	let em1 = windowManager.emCoord(1);
	let em20 = windowManager.emCoord(20);
	let submitTextWidth = windowManager.strWidth("Submit", 1);
	windowManager.addWindow(Window::new("Window 1", elementSpacing), RectArea::new(50, 50, 480, 320))
		.addElement(em20, 4 + em1 + verticalBorderSize, Box::new(TextInput::new()))
		.addElement(submitTextWidth, 4 + em1 + verticalBorderSize, Box::new(Button::new(String::from("Submit"))));
	windowManager.addWindow(Window::new("Window 2", elementSpacing), RectArea::new(550, 25, 200, 200));
	windowManager.addWindow(Window::new("Window 3", elementSpacing), RectArea::new(550, 250, 200, 325));

	let mut event_pump = sdl_context.event_pump().unwrap();
	video_subsystem.text_input().start();

	'running: loop
	{
		for event in event_pump.poll_iter()
		{
			windowManager.handleInput(&event);
		}

		if windowManager.quit
		{
			break 'running;
		}

		windowManager.draw();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}