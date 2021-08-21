use sdl2::rect::Rect;

#[derive(Copy, Clone, Debug)]
pub struct XY
{
	pub x: i32,
	pub y: i32
}

impl XY
{
	pub fn offset(&self, xrel: i32, yrel: i32) -> XY
	{
		return XY { x: self.x + xrel, y: self.y + yrel };
	}
}

#[derive(Copy, Clone, Debug)]
pub struct RectArea
{
	pub pos: XY,
	pub siz: XY
}

impl RectArea
{
	pub fn intersects(&self, other: RectArea) -> bool
	{
		return !(self.pos.x > other.pos.x + other.siz.x
				|| self.pos.x + self.siz.x < other.pos.x
				|| self.pos.y > other.pos.y + other.siz.y
				|| self.pos.y + self.siz.y < other.pos.y
		);
	}

	pub fn adjusted(&self, xrel: i32, yrel: i32, widthRel: i32, heightRel: i32) -> RectArea
	{
		let result = RectArea { pos: self.pos.offset(xrel, yrel), siz: self.siz.offset(widthRel, heightRel)};
		return result;
	}
	
	pub fn to_rect(&self) -> Rect
	{
		return Rect::new(self.pos.x, self.pos.y, self.siz.x as u32, self.siz.y as u32);
	}
}