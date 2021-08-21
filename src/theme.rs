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
			right: w98Theme3dBoxBorder
			{
				inner: self.left.outer,
				outer: self.left.inner
			},
			bottom: w98Theme3dBoxBorder
			{
				inner: self.top.outer,
				outer: self.top.inner
			}
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

pub static CLASSIC_W98_THEME: w98Theme = basicClassicW98Theme(
	"C:/WINDOWS/FONTS/ARIAL.TTF", 			// Font Path
	16,										// Font Points
	1,										// Bar width for all [3d]ThemeBar widths
	Color::RGB(0, 127, 127), 				// Base Background
	Color::RGB(191, 191, 191), 				// Window Background
	Color::RGB(0, 0, 127),					// Title Active Background
	Color::RGB(127, 127, 127),				// Title Inactive Background
	Color::WHITE,							// Font color
	Color::RGB(223, 223, 223), 				// 3d Box Light Border
	Color::RGB(255, 255, 255), 				// 3d Box Very Light Border
	Color::RGB(127, 127, 127), 				// 3d Box Dark Border
	Color::RGB(0, 0, 0)						// 3d Box Very Dark Border
);

pub static MYSTERY_W98_THEME: w98Theme = basicClassicW98Theme(
	CLASSIC_W98_THEME.font.path, 			// Font Path
	CLASSIC_W98_THEME.font.points,			// Font Points
	1,										// Bar width for all [3d]ThemeBar widths
	Color::RGB(0, 127, 127), 				// Base Background
	Color::RGB(103, 120, 104), 				// Window Background
	Color::RGB(100, 133, 52),				// Title Active Background
	Color::RGB(89, 89, 89),					// Title Inactive Background
	Color::WHITE,							// Font color
	Color::RGB(152, 166, 153), 				// 3d Box Light Border
	Color::RGB(152, 166, 153), 				// 3d Box Very Light Border
	Color::RGB(41, 44, 35), 				// 3d Box Dark Border
	Color::RGB(0, 0, 0)						// 3d Box Very Dark Border
);