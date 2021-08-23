use sdl2::rect::Rect;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

pub const ORIGIN: XY = XY { x: 0, y: 0 };

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct RectArea
{
	pub x: i32,		// Horizontal Offset from Origin
	pub y: i32,		// Vertical Offset from Origin
	pub w: u32,		// Width
	pub h: u32		// Height
}

pub const ORIGIN_ZERO: RectArea = RectArea::new(0, 0, 0, 0);

impl RectArea
{
	pub const fn new(x: i32, y: i32, w: u32, h: u32) -> RectArea
	{
		return RectArea{ x, y, w, h };
	}

	pub fn intersects(&self, other: RectArea) -> bool
	{
		self.intersects2(other.x, other.y, other.w, other.h)
	}
	
	pub fn intersects2(&self, x: i32, y: i32, w: u32, h: u32) -> bool
	{
		return !(self.x > x + w as i32
			|| self.x + (self.w as i32) < x
			|| self.y > y + h as i32
			|| self.y + (self.h as i32) < y
		);
	}

	pub fn adjusted(&self, xrel: i32, yrel: i32, widthRel: i32, heightRel: i32) -> RectArea
	{
		return RectArea::new(self.x + xrel, self.y + yrel, ((self.w as i32) + widthRel) as u32, ((self.h as i32) + heightRel) as u32);
	}
	
	pub fn to_rect(&self) -> Rect
	{
		return Rect::new(self.x, self.y, self.w, self.h);
	}
}