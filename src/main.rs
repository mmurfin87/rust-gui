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
	//let w1ra = RectArea::new(50, 50, 480, 320);// { pos: XY {x: 50, y: 50}, siz: XY {x: 480, y: 320}};
	//let w1tira = era(&w1ra, &windowManager.theme, 10, 30, 20);
	//let w1bra = era(&w1ra, &windowManager.theme, 10, 60, 7);
	//let w2ra = RectArea::new(550, 25, 200, 200);// { pos: XY {x: 550, y: 25}, siz: XY {x: 200, y: 200}};
	//let w3ra = RectArea::new(550, 250, 200, 325);// { pos: XY {x: 550, y: 250}, siz: XY {x: 200, y: 325}};
	//windowManager.windows.push(Window{name: "Window 1", area: w1ra, lastArea: w1ra, focus: true, focusedElement: -1, elements: vec![Element { width: w1ra.siz.x, height: w1ra.siz.y, behavior: Box::new(TextInput { area: w1tira, text: "".to_owned(), cursorIndex: 0, frames: 0 }), Box::new(Button { area: w1bra, text: "Submit".to_owned(), pressed: false,})]}});
	//windowManager.windows.push(Window{name: "Window 2", area: w2ra, lastArea: w2ra, focus: false, focusedElement: -1, elements: std::vec::Vec::new()});
	//windowManager.windows.push(Window{name: "Window 3", area: w3ra, lastArea: w3ra, focus: false, focusedElement: -1, elements: std::vec::Vec::new()});
	let elementSpacing = windowManager.theme.font.points as u32 + 4;
	let w1 = Window::new("Window 1", elementSpacing);
	let verticalBorderSize = windowManager.verticalBorderSize();
	let em1 = windowManager.emCoord(1);
	let em20 = windowManager.emCoord(20);
	windowManager.addWindow(w1, RectArea::new(50, 50, 480, 320))
		.addElement(em20, 4 + em1 + verticalBorderSize, Box::new(TextInput::new()))
		.addElement(em20, 4 + em1 + verticalBorderSize, Box::new(Button::new(String::from("Submit"))));
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

fn era(area: &RectArea, theme: &w98Theme, offsetX: i32, offsetY: i32, widthEms: u32) -> RectArea
{
	return RectArea::new(
		area.x + offsetX, 
		area.y + offsetY, 
		theme.font.points as u32 * widthEms, 
		4 + theme.font.points as u32 + theme.window.border.top.outer.width + theme.window.border.top.inner.width + theme.window.border.bottom.inner.width + theme.window.border.bottom.outer.width
	);
}