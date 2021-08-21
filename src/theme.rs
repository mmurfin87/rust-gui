extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::ttf::FontStyle;

#[derive(Copy, Clone)]
pub struct w98ThemeBar
{
	pub color: Color,
	pub inactiveColor: Color,
	pub width: u32
}

#[derive(Copy, Clone)]
pub struct w98Theme3dBoxBorder
{
	pub inner: w98ThemeBar,
	pub outer: w98ThemeBar
}

pub struct w98ThemeFlatBox
{
	pub top: w98ThemeBar,
	pub left: w98ThemeBar,
	pub right: w98ThemeBar,
	pub bottom: w98ThemeBar
}

pub struct w98Theme3dBox
{
	pub top: w98Theme3dBoxBorder,
	pub left: w98Theme3dBoxBorder,
	pub right: w98Theme3dBoxBorder,
	pub bottom: w98Theme3dBoxBorder
}

impl w98Theme3dBox
{
	pub fn invert(&self) -> w98Theme3dBox
	{
		return w98Theme3dBox
		{
			top: self.bottom,
			left: self.right,
			right: self.left,
			bottom: self.top
		}
	}
}

pub struct w98ThemeFont
{
	pub path: &'static str,
	pub points: u16,
	pub style: FontStyle,
	pub color: Color
}

pub struct w98ThemeWindow
{
	pub backgroundColor: Color,
	pub titleFont: w98ThemeFont,
	pub titleBorder: w98ThemeFlatBox,
	pub titleBar: w98ThemeBar,
	pub border: w98Theme3dBox
}

pub struct w98Theme
{
	pub font: w98ThemeFont,
	pub backgroundColor: Color,
	pub window: w98ThemeWindow
}

pub const fn basicClassicW98Theme(fontPath: &'static str, fontPoints: u16, barWidth: u32, bgColor: Color, windowBgColor: Color, titleBgColor: Color, titleInactiveBgColor: Color, textColor: Color, lightBorder: Color, veryLightBorder: Color, darkBorder: Color, veryDarkBorder: Color) -> w98Theme
{
	return w98Theme
	{
		font: w98ThemeFont
		{
			path: fontPath,
			points: fontPoints,
			style: FontStyle::NORMAL,
			color: textColor,
		},
		backgroundColor: bgColor,
		window: w98ThemeWindow
		{
			backgroundColor: windowBgColor,
			titleFont: w98ThemeFont
			{
				path: fontPath,
				points: fontPoints,
				style: FontStyle::NORMAL,
				color: textColor
			},
			titleBorder: w98ThemeFlatBox
			{
				top: w98ThemeBar
				{
					color: windowBgColor,
					inactiveColor: windowBgColor,
					width: barWidth
				},
				left: w98ThemeBar
				{
					color: windowBgColor,
					inactiveColor: windowBgColor,
					width: barWidth
				},
				right: w98ThemeBar
				{
					color: windowBgColor,
					inactiveColor: windowBgColor,
					width: barWidth
				},
				bottom: w98ThemeBar
				{
					color: windowBgColor,
					inactiveColor: windowBgColor,
					width: barWidth
				}
			},
			titleBar: w98ThemeBar
			{
				color: titleBgColor,
				inactiveColor: titleInactiveBgColor,
				width: fontPoints as u32 + (fontPoints as u32 / 3)
			},
			border: w98Theme3dBox
			{
				top: w98Theme3dBoxBorder
				{
					inner: w98ThemeBar
					{
						color: veryLightBorder,
						inactiveColor: veryLightBorder,
						width: barWidth
					},
					outer: w98ThemeBar
					{
						color: lightBorder,
						inactiveColor: lightBorder,
						width: barWidth
					}
	
				},
				left: w98Theme3dBoxBorder
				{
					inner: w98ThemeBar
					{
						color: veryLightBorder,
						inactiveColor: veryLightBorder,
						width: barWidth
					},
					outer: w98ThemeBar
					{
						color: lightBorder,
						inactiveColor: lightBorder,
						width: barWidth
					}
				},
				right: w98Theme3dBoxBorder
				{
					inner: w98ThemeBar
					{
						color: veryDarkBorder,
						inactiveColor: veryDarkBorder,
						width: barWidth
					},
					outer: w98ThemeBar
					{
						color: darkBorder,
						inactiveColor: darkBorder,
						width: barWidth
					}
				},
				bottom: w98Theme3dBoxBorder
				{
					inner: w98ThemeBar
					{
						color: veryDarkBorder,
						inactiveColor: veryDarkBorder,
						width: barWidth
					},
					outer: w98ThemeBar
					{
						color: darkBorder,
						inactiveColor: darkBorder,
						width: barWidth
					}
				},
			}
		}
	};
}

pub static CLASSIC_W98_THEME_BAR_WIDTH: u32 = 1;
pub static CLASSIC_W98_THEME_BASE_BACKGROUND: Color = Color::RGB(0, 127, 127);
pub static CLASSIC_W98_THEME_WINDOW_BACKGROUND: Color = Color::RGB(191, 191, 191);
pub static CLASSIC_W98_THEME_TITLE_BACKGROUND: Color = Color::RGB(0, 0, 127);
pub static CLASSIC_W98_THEME_TITLE_INACTIVE_BACKGROUND: Color = Color::RGB(127, 127, 127);
pub static CLASSIC_W98_THEME_FONT_PATH: &str = "C:/WINDOWS/FONTS/ARIAL.TTF";
pub static CLASSIC_W98_THEME_TEXT_COLOR: Color = Color::WHITE;
pub static CLASSIC_W98_THEME_TEXT_POINTS: u16 = 16;
pub static CLASSIC_W98_THEME_LIGHT_BORDER: Color = Color::RGB(223, 223, 223);
pub static CLASSIC_W98_THEME_VERY_LIGHT_BORDER: Color = Color::RGB(255, 255, 255);
pub static CLASSIC_W98_THEME_DARK_BORDER: Color = Color::RGB(127, 127, 127);
pub static CLASSIC_W98_THEME_VERY_DARK_BORDER: Color = Color::RGB(0, 0, 0);
pub static CLASSIC_W98_THEME: w98Theme = basicClassicW98Theme(
	CLASSIC_W98_THEME_FONT_PATH, 
	CLASSIC_W98_THEME_TEXT_POINTS,
	CLASSIC_W98_THEME_BAR_WIDTH,
	CLASSIC_W98_THEME_BASE_BACKGROUND, 
	CLASSIC_W98_THEME_WINDOW_BACKGROUND, 
	CLASSIC_W98_THEME_TITLE_BACKGROUND,
	CLASSIC_W98_THEME_TITLE_INACTIVE_BACKGROUND,
	CLASSIC_W98_THEME_TEXT_COLOR,
	CLASSIC_W98_THEME_LIGHT_BORDER, 
	CLASSIC_W98_THEME_VERY_LIGHT_BORDER, 
	CLASSIC_W98_THEME_DARK_BORDER, 
	CLASSIC_W98_THEME_VERY_DARK_BORDER);

pub static MYSTERY_W98_THEME_BAR_WIDTH:u32 = CLASSIC_W98_THEME_BAR_WIDTH;
pub static MYSTERY_W98_THEME_BASE_BACKGROUND:Color = Color::RGB(0, 127, 127);
pub static MYSTERY_W98_THEME_WINDOW_BACKGROUND: Color = Color::RGB(103, 120, 104);
pub static MYSTERY_W98_THEME_TITLE_BACKGROUND: Color = Color::RGB(100, 133, 52);
pub static MYSTERY_W98_THEME_TITLE_INACTIVE_BACKGROUND: Color = Color::RGB(89, 89, 89);
pub static MYSTERY_W98_THEME_FONT_PATH: &str = "C:/WINDOWS/FONTS/ARIAL.TTF";
pub static MYSTERY_W98_THEME_TEXT_COLOR: Color = Color::WHITE;
pub static MYSTERY_W98_THEME_TEXT_POINTS: u16 = CLASSIC_W98_THEME_TEXT_POINTS;
pub static MYSTERY_W98_THEME_VERY_LIGHT_BORDER: Color = Color::RGB(152, 166, 153);
pub static MYSTERY_W98_THEME_LIGHT_BORDER: Color = Color::RGB(102, 116, 103);
pub static MYSTERY_W98_THEME_DARK_BORDER: Color = Color::RGB(41, 44, 35);
pub static MYSTERY_W98_THEME_VERY_DARK_BORDER: Color = Color::RGB(0, 0, 0);
pub static MYSTERY_W98_THEME: w98Theme = basicClassicW98Theme(
	MYSTERY_W98_THEME_FONT_PATH, 
	MYSTERY_W98_THEME_TEXT_POINTS,
	MYSTERY_W98_THEME_BAR_WIDTH,
	MYSTERY_W98_THEME_BASE_BACKGROUND, 
	MYSTERY_W98_THEME_WINDOW_BACKGROUND, 
	MYSTERY_W98_THEME_TITLE_BACKGROUND,
	MYSTERY_W98_THEME_TITLE_INACTIVE_BACKGROUND,
	MYSTERY_W98_THEME_TEXT_COLOR,
	MYSTERY_W98_THEME_LIGHT_BORDER, 
	MYSTERY_W98_THEME_VERY_LIGHT_BORDER, 
	MYSTERY_W98_THEME_DARK_BORDER, 
	MYSTERY_W98_THEME_VERY_DARK_BORDER);