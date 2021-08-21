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
	let texture_creator = canvas.texture_creator();

	let mut windowManager = WindowManager { quit: false, windows: std::vec::Vec::new(), theme: &CLASSIC_W98_THEME, useClassicTheme: true, capturedWindow: false };

	let mut font = ttf_context.load_font(windowManager.theme.font.path, windowManager.theme.font.points).unwrap();
	font.set_style(sdl2::ttf::FontStyle::NORMAL);
	let mut drawContext = DrawContext{canvas, font, texture_creator};
	
	let w1ra = RectArea { pos: XY {x: 50, y: 50}, siz: XY {x: 480, y: 320}};
	let w1tira = era(&w1ra, &windowManager.theme, 10, 30, 20);
	let w1bra = era(&w1ra, &windowManager.theme, 10, 60, 7);
	windowManager.windows.push(Window{name: "Window 1", area: w1ra, focus: true, elements: vec![Box::new(TextInput { area: w1tira, text: "".to_owned(), cursorIndex: 0, frames: 0 }), Box::new(Button { area: w1bra, text: "Submit".to_owned(), pressed: false})]});
	windowManager.windows.push(Window{name: "Window 2", area: RectArea { pos: XY {x: 550, y: 25}, siz: XY {x: 200, y: 200}}, focus: false, elements: std::vec::Vec::new()});
	windowManager.windows.push(Window{name: "Window 3", area: RectArea { pos: XY {x: 550, y: 250}, siz: XY {x: 200, y: 325}}, focus: false, elements: std::vec::Vec::new()});

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

		windowManager.draw(&mut drawContext);

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}

const fn era(area: &RectArea, theme: &w98Theme, offsetX: i32, offsetY: i32, widthEms: i32) -> RectArea
{
	return RectArea
	{
		pos: XY
		{
			x: area.pos.x + offsetX,
			y: area.pos.y + offsetY
		},
		siz: XY
		{
			x: theme.font.points as i32 * widthEms,
			y: (theme.font.points as u32 + theme.window.border.top.outer.width + theme.window.border.top.inner.width + theme.window.border.bottom.inner.width + theme.window.border.bottom.outer.width) as i32
		}
	};
}