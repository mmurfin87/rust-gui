#[derive(Copy, Clone)]
pub struct XY
{
	pub x: i32,
	pub y: i32
}

#[derive(Copy, Clone)]
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
}