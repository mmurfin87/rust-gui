use super::super::theme::*;
use super::super::rectarea::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::render::TextureQuery;

pub fn innerRectArea(theme: &w98Theme3dBox, w: &RectArea) -> RectArea
{
	return RectArea
	{
		pos: XY
		{ 
			x: w.pos.x + theme.left.outer.width as i32 + theme.left.inner.width as i32, 
			y: w.pos.y + theme.top.outer.width as i32 + theme.top.inner.width as i32
		},
		siz: XY 
		{ 
			x: w.siz.x - theme.left.outer.width as i32 - theme.left.inner.width as i32 - theme.right.inner.width as i32 - theme.right.outer.width as i32, 
			y: w.siz.y - theme.top.outer.width as i32 - theme.top.inner.width as i32 - theme.bottom.inner.width as i32 - theme.bottom.outer.width as i32
		}
	};
}

pub fn drawW983dBox<T: sdl2::render::RenderTarget>(canvas: &mut Canvas<T>, theme: &w98Theme3dBox, backgroundColor: Color, w: &RectArea)
{
	canvas.set_draw_color(theme.top.outer.color);
	canvas.fill_rect(Rect::new(w.pos.x, w.pos.y, w.siz.x as u32 - theme.right.outer.width, theme.top.inner.width)).expect("windowTopOuterBorder");
	canvas.set_draw_color(theme.top.inner.color);
	canvas.fill_rect(Rect::new(w.pos.x + theme.top.inner.width as i32, w.pos.y + theme.left.outer.width as i32, w.siz.x as u32 - theme.left.outer.width - theme.right.inner.width - theme.right.outer.width, theme.top.inner.width)).expect("windowTopInnerBorder");
	
	canvas.set_draw_color(theme.left.outer.color);
	canvas.fill_rect(Rect::new(w.pos.x, w.pos.y + theme.top.outer.width as i32, theme.left.outer.width, w.siz.y as u32 - theme.top.outer.width - theme.bottom.outer.width)).expect("windowLeftOuterBorder");
	canvas.set_draw_color(theme.left.inner.color);
	canvas.fill_rect(Rect::new(w.pos.x + theme.left.outer.width as i32, w.pos.y + theme.top.inner.width as i32 + theme.top.outer.width as i32, theme.left.inner.width, w.siz.y as u32 - theme.top.inner.width - theme.top.outer.width - theme.bottom.inner.width - theme.bottom.outer.width)).expect("windowLeftInnerBorder");
	
	canvas.set_draw_color(theme.right.outer.color);
	canvas.fill_rect(Rect::new(w.pos.x + w.siz.x - theme.right.outer.width as i32, w.pos.y, theme.right.outer.width, w.siz.y as u32 - theme.bottom.outer.width)).expect("windowRightOuterBorder");
	canvas.set_draw_color(theme.right.inner.color);
	canvas.fill_rect(Rect::new(w.pos.x + w.siz.x - theme.right.inner.width as i32 - theme.right.outer.width as i32, w.pos.y + theme.top.outer.width as i32, theme.right.inner.width, w.siz.y as u32 - theme.top.outer.width - theme.bottom.inner.width - theme.bottom.outer.width)).expect("windowRightInnerBorder");
	
	canvas.set_draw_color(theme.bottom.outer.color);
	canvas.fill_rect(Rect::new(w.pos.x, w.pos.y + w.siz.y - theme.bottom.outer.width as i32, w.siz.x as u32, theme.bottom.outer.width)).expect("windowBottomOuterBorder");
	canvas.set_draw_color(theme.bottom.inner.color);
	canvas.fill_rect(Rect::new(w.pos.x + theme.left.outer.width as i32, w.pos.y + w.siz.y - theme.bottom.inner.width as i32 - theme.bottom.outer.width as i32, w.siz.x as u32 - theme.left.outer.width - theme.right.outer.width, theme.bottom.inner.width)).expect("windowBottomInnerBorder");

	canvas.set_draw_color(backgroundColor);
	canvas.fill_rect(innerRectArea(theme, w).to_rect()).expect("windowBackground");
}

pub fn writeLeftAligned<T: sdl2::render::RenderTarget>(canvas: &mut Canvas<T>, textureCreator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, font: &Font, color: &Color, origin: XY, maxWidth: u32, text: &str)
{
	let surface = font.render(text)
		.blended(*color)
		.unwrap();
	let texture = textureCreator.create_texture_from_surface(&surface).unwrap();
	let TextureQuery { width, height, .. } = texture.query();
	let skip: u32 = if width > maxWidth { width - maxWidth } else { 0 };
	canvas.copy(&texture, Some(Rect::new(skip as i32, 0, width - skip, height)), Some(Rect::new(origin.x, origin.y, width - skip, height))).expect("writeLeftAligned");
}