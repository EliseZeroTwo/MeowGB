use super::interrupts::Interrupts;

pub const FB_HEIGHT: u32 = 144;
pub const FB_WIDTH: u32 = 160;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
	id3: Color,
	id2: Color,
	id1: Color,
	id0: Color,
}

impl Palette {
	pub fn color_from_2bit(&self, value: u8) -> Color {
		match value & 0b11 {
			0 => self.id0,
			1 => self.id1,
			2 => self.id2,
			3 => self.id3,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineDrawingState {
	/// (Cycles left, SCX, SCY)
	BackgroundScrolling(usize, u8, u8),
	/// (original SCX, original SCY, drawn pixel count, window drawn, draw only
	/// sprites)
	BackgroundAndObjectFifo(u8, u8, u8, bool, bool),
	Finished,
}

impl Palette {
	pub fn new_bgp() -> Self {
		Self { id0: Color::White, id1: Color::LGray, id2: Color::DGray, id3: Color::Black }
	}

	pub fn new_obp() -> Self {
		Self { id0: Color::Transparent, id1: Color::LGray, id2: Color::DGray, id3: Color::Black }
	}

	pub fn write_bgp(&mut self, value: u8) {
		self.id0 = Color::from_bg_2bit(value);
		self.id1 = Color::from_bg_2bit(value >> 2);
		self.id2 = Color::from_bg_2bit(value >> 4);
		self.id3 = Color::from_bg_2bit(value >> 6);
	}

	pub fn write_obp(&mut self, value: u8) {
		self.id0 = Color::from_obj_2bit(value);
		self.id1 = Color::from_obj_2bit(value >> 2);
		self.id2 = Color::from_obj_2bit(value >> 4);
		self.id3 = Color::from_obj_2bit(value >> 6);
	}

	pub fn value(&self) -> u8 {
		(self.id0.to_2bit())
			| (self.id1.to_2bit() << 2)
			| (self.id2.to_2bit() << 4)
			| (self.id3.to_2bit() << 6)
	}
}

pub struct WrappedBuffer<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> std::ops::Index<usize> for WrappedBuffer<SIZE> {
	type Output = u8;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index % SIZE]
	}
}

impl<const SIZE: usize> std::ops::IndexMut<usize> for WrappedBuffer<SIZE> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index % SIZE]
	}
}

impl<const SIZE: usize> WrappedBuffer<SIZE> {
	pub fn empty() -> Self {
		Self([0; SIZE])
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PPUMode {
	/// Mode 0
	HBlank,
	/// Mode 1
	VBlank,
	/// Mode 2
	SearchingOAM,
	/// Mode 3
	TransferringData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
	White,
	LGray,
	DGray,
	Black,
	Transparent,
}

impl Color {
	const WHITE: [u8; 4] = [0xe0, 0xf8, 0xd0, 0xFF];
	const LGRAY: [u8; 4] = [0x88, 0xc0, 0x70, 0xFF];
	const DGRAY: [u8; 4] = [0x34, 0x68, 0x56, 0xFF];
	const BLACK: [u8; 4] = [0x08, 0x18, 0x20, 0xFF];
	const TRANSPARENT: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
	pub fn rgba(self) -> &'static [u8; 4] {
		match self {
			Color::White => &Self::WHITE,
			Color::LGray => &Self::LGRAY,
			Color::DGray => &Self::DGRAY,
			Color::Black => &Self::BLACK,
			Color::Transparent => &Self::TRANSPARENT,
		}
	}

	#[allow(unused)]
	pub fn from_rgba(rgba: [u8; 4]) -> Option<Self> {
		match rgba {
			Self::WHITE => Some(Self::White),
			Self::LGRAY => Some(Self::LGray),
			Self::DGRAY => Some(Self::DGray),
			Self::BLACK => Some(Self::Black),
			Self::TRANSPARENT => Some(Self::Transparent),
			_ => None,
		}
	}

	pub fn from_bg_2bit(value: u8) -> Self {
		match value & 0b11 {
			0 => Self::White,
			1 => Self::LGray,
			2 => Self::DGray,
			3 => Self::Black,
			_ => unreachable!(),
		}
	}

	pub fn from_obj_2bit(value: u8) -> Self {
		match value & 0b11 {
			0 => Self::Transparent,
			1 => Self::LGray,
			2 => Self::DGray,
			3 => Self::Black,
			_ => unreachable!(),
		}
	}

	pub fn to_2bit(&self) -> u8 {
		match self {
			Color::White => 0,
			Color::LGray => 1,
			Color::DGray => 2,
			Color::Black => 3,
			Color::Transparent => 0,
		}
	}

	pub fn parse_bgp_color(low: u8, high: u8, palette: &Palette) -> (u8, Self) {
		let color = ((high & 0b1) << 1) | low & 0b1;
		match color & 0b11 {
			0 => (color & 0b11, palette.id0),
			1 => (color & 0b11, palette.id1),
			2 => (color & 0b11, palette.id2),
			3 => (color & 0b11, palette.id3),
			_ => unreachable!(),
		}
	}

	pub fn parse_bgp(mut bgp_low: u8, mut bgp_high: u8, palette: &Palette) -> [Self; 8] {
		let mut out = [Self::White; 8];
		for color in &mut out {
			*color = Self::parse_bgp_color(bgp_low, bgp_high, palette).1;
			bgp_low >>= 1;
			bgp_high >>= 1;
		}
		out.reverse();
		out
	}
}

#[derive(Debug, Clone, Copy)]
pub struct OAMEntry {
	pub y: u8,
	pub x: u8,
	pub tile_idx: u8,
	pub flags: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SpriteHeight {
	Eight = 8,
	Sixteen = 16,
}

impl OAMEntry {
	pub fn parse(entry: [u8; 4]) -> Self {
		Self { y: entry[0], x: entry[1], tile_idx: entry[2], flags: entry[3] }
	}

	pub fn covered_by_bg_window(&self) -> bool {
		(self.flags >> 7) & 0b1 == 1
	}

	pub fn y_flip(&self) -> bool {
		(self.flags >> 6) & 0b1 == 1
	}

	pub fn x_flip(&self) -> bool {
		(self.flags >> 5) & 0b1 == 1
	}

	pub fn palette_number(&self) -> bool {
		(self.flags >> 4) & 0b1 == 1
	}
}

impl PPUMode {
	pub fn mode_flag(&self) -> u8 {
		match self {
			PPUMode::HBlank => 0,
			PPUMode::VBlank => 1,
			PPUMode::SearchingOAM => 2,
			PPUMode::TransferringData => 3,
		}
	}
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StatFlags {
	pub lyc_int: bool,
	pub mode2_int: bool,
	pub mode1_int: bool,
	pub mode0_int: bool,
}

impl StatFlags {
	pub fn flag_bits(&self) -> u8 {
		((self.lyc_int as u8) << 6)
			| ((self.mode2_int as u8) << 5)
			| ((self.mode1_int as u8) << 4)
			| ((self.mode0_int as u8) << 3)
	}

	pub fn from_bits(&mut self, stat: u8) {
		self.lyc_int = (stat >> 6) & 1 == 1;
		self.mode2_int = (stat >> 5) & 1 == 1;
		self.mode1_int = (stat >> 4) & 1 == 1;
		self.mode0_int = (stat >> 3) & 1 == 1;
	}
}

pub struct Ppu {
	pub lcdc: u8,
	stat_flags: StatFlags,
	mode: PPUMode,
	pub scy: u8,
	pub scx: u8,
	pub ly: u8,
	pub lyc: u8,
	pub wy: u8,
	pub wx: u8,

	pub vram: [u8; 0x2000],
	pub oam: [u8; 0xA0],

	pub bgp: Palette,
	pub obp: [Palette; 2],

	pub framebuffer: WrappedBuffer<{ 160 * 144 * 4 }>,
	pub sprite_framebuffer: WrappedBuffer<{ 160 * 144 * 4 }>,

	ly_lyc: bool,

	// Internals
	current_dot: u16,
	dot_target: u16,

	sprite_buffer: [Option<OAMEntry>; 10],
	sprite_count: usize,

	pub dma_occuring: bool,

	current_draw_state: Option<LineDrawingState>,
	wy_match: bool,

	first_frame: bool,
	first_line: bool,
	total_dots: u16,

	is_irq_high: bool,
}

impl Ppu {
	pub fn stop(&mut self) {
		self.current_dot = 0;
		self.dot_target = 0;
		self.sprite_buffer = [None; 10];
		self.sprite_count = 0;
		self.current_draw_state = None;
		self.wy_match = false;
		self.mode = PPUMode::HBlank;
		self.first_frame = true;
		self.first_line = true;
		self.total_dots = 0;
	}

	pub fn start(&mut self, interrupts: &mut Interrupts) {
		self.set_scanline(interrupts, 0);
	}

	pub fn new(bootrom_ran: bool) -> Self {
		Self {
			lcdc: (!bootrom_ran).then_some(0b1000_0000).unwrap_or_default(),
			stat_flags: StatFlags::default(),
			mode: PPUMode::HBlank,
			scy: 0,
			scx: 0,
			ly: 0,
			lyc: 0,
			wy: 0,
			wx: 0,
			vram: [0; 0x2000],
			oam: [0; 0xA0],
			framebuffer: WrappedBuffer::empty(),
			sprite_framebuffer: WrappedBuffer::empty(),
			bgp: Palette::new_bgp(),
			obp: [Palette::new_obp(), Palette::new_obp()],

			ly_lyc: true,

			current_dot: 0,
			dot_target: 0,
			sprite_buffer: [None; 10],
			sprite_count: 0,
			dma_occuring: false,
			current_draw_state: None,
			wy_match: false,
			first_frame: true,
			total_dots: 0,
			first_line: true,
			is_irq_high: false,
		}
	}

	pub fn handle_stat_irq(&mut self, interrupts: &mut Interrupts) {
		if self.enabled() {
			let old_irq_high = self.is_irq_high;

			let stat_int = match self.mode {
				PPUMode::HBlank => self.stat_flags.mode0_int,
				PPUMode::VBlank => self.stat_flags.mode1_int,
				PPUMode::SearchingOAM => self.stat_flags.mode2_int,
				PPUMode::TransferringData => false,
			};

			let vblank_causing_oam_int_bug =
				self.mode == PPUMode::VBlank && self.ly == 144 && self.stat_flags.mode2_int;

			let ly_eq_lyc_int = self.ly == self.lyc && self.stat_flags.lyc_int;

			self.is_irq_high = stat_int || ly_eq_lyc_int || vblank_causing_oam_int_bug;

			if !old_irq_high && self.is_irq_high {
				interrupts.write_if_lcd_stat(true);
			}
		}
	}

	pub fn set_lyc(&mut self, interrupts: &mut Interrupts, value: u8) {
		if self.lyc != value {
			self.lyc = value;
			if self.enabled() {
				self.handle_stat_irq(interrupts);
				self.ly_lyc = self.ly == self.lyc;
			}
		}
	}

	pub fn get_stat(&self) -> u8 {
		let ly_eq_lyc = match self.enabled() {
			true => self.ly == self.lyc,
			false => self.ly_lyc,
		};
		(1 << 7) | self.stat_flags.flag_bits() | ((ly_eq_lyc as u8) << 2) | self.mode().mode_flag()
	}

	pub fn set_stat(&mut self, interrupts: &mut Interrupts, value: u8) {
		self.stat_flags.from_bits(value);
		self.handle_stat_irq(interrupts);
	}

	pub fn sprite_height(&self) -> SpriteHeight {
		match (self.lcdc >> 2) == 1 {
			false => SpriteHeight::Eight,
			true => SpriteHeight::Sixteen,
		}
	}

	pub fn mode(&self) -> PPUMode {
		self.mode
	}

	pub fn read_window_tile_map(&self) -> &[u8] {
		match (self.lcdc >> 6) & 0b1 == 1 {
			true => &self.vram[0x1C00..=0x1FFF],
			false => &self.vram[0x1800..=0x1BFF],
		}
	}

	pub fn read_tile_map(&self) -> &[u8] {
		match (self.lcdc >> 3) & 0b1 == 1 {
			true => &self.vram[0x1C00..=0x1FFF],
			false => &self.vram[0x1800..=0x1BFF],
		}
	}

	pub fn read_obj_tile_colour_id(&self, tile_idx: u8, x: usize, y: usize) -> u8 {
		assert!(x < 8);
		assert!(y < 8);
		let bitshift = 7 - x;
		let offset = (tile_idx as usize * 16) + (y * 2);
		let low = self.vram[offset] >> bitshift;
		let high = self.vram[offset + 1] >> bitshift;
		((high & 0b1) << 1) | low & 0b1
	}

	fn internal_read_oam(&mut self, offset: usize) -> u8 {
		match self.dma_occuring {
			true => 0xFF,
			false => self.oam[offset as usize],
		}
	}

	pub fn dma_write_oam(&mut self, offset: u8, value: u8) {
		assert!(self.dma_occuring);
		self.oam[offset as usize] = value;
	}

	pub fn cpu_read_oam(&self, address: u16) -> u8 {
		let decoded_address = address - 0xFE00;
		if self.enabled() && !self.first_frame {
			match self.mode() {
				PPUMode::HBlank | PPUMode::VBlank => self.oam[decoded_address as usize],
				PPUMode::SearchingOAM | PPUMode::TransferringData => 0xFF,
			}
		} else {
			self.oam[decoded_address as usize]
		}
	}

	pub fn cpu_write_oam(&mut self, address: u16, value: u8) {
		let decoded_address = address - 0xFE00;
		if self.enabled() && !self.first_frame {
			match self.mode() {
				PPUMode::HBlank | PPUMode::VBlank => self.oam[decoded_address as usize] = value,
				PPUMode::SearchingOAM | PPUMode::TransferringData => {}
			}
		} else {
			self.oam[decoded_address as usize] = value
		}
	}

	pub fn cpu_read_vram(&self, address: u16) -> u8 {
		let decoded_address = address - 0x8000;
		if self.enabled() && !self.first_frame {
			match self.mode() {
				PPUMode::HBlank | PPUMode::VBlank | PPUMode::SearchingOAM => {
					self.vram[decoded_address as usize]
				}
				PPUMode::TransferringData => 0xFF,
			}
		} else {
			self.vram[decoded_address as usize]
		}
	}

	pub fn cpu_write_vram(&mut self, address: u16, value: u8) {
		let decoded_address = address - 0x8000;
		if self.enabled() && !self.first_frame {
			match self.mode() {
				PPUMode::HBlank | PPUMode::VBlank | PPUMode::SearchingOAM => {
					self.vram[decoded_address as usize] = value
				}
				PPUMode::TransferringData => {}
			}
		} else {
			self.vram[decoded_address as usize] = value
		}
	}

	pub fn enabled(&self) -> bool {
		(self.lcdc >> 7) == 1
	}

	pub fn set_mode(&mut self, interrupts: &mut Interrupts, mode: PPUMode) {
		if mode == PPUMode::HBlank {
			assert_eq!(self.mode(), PPUMode::TransferringData);
			assert!(self.current_dot >= 172);
			assert!(self.current_dot <= 289);
			self.dot_target = 376 - self.dot_target;
			assert!(self.dot_target >= 87);
			assert!(self.dot_target <= 204);
		} else if mode == PPUMode::TransferringData {
			if !self.first_frame {
				assert_eq!(self.mode(), PPUMode::SearchingOAM);
			} else if self.ly == 0 {
				assert_eq!(self.mode(), PPUMode::HBlank);
			}
			self.current_draw_state = None;
			self.dot_target = 160 + 12;
		}

		self.mode = mode;
		self.current_dot = 0;

		self.handle_stat_irq(interrupts);

		if mode == PPUMode::VBlank {
			interrupts.write_if_vblank(true);
		}
	}

	fn set_scanline(&mut self, interrupts: &mut Interrupts, scanline: u8) {
		self.ly = scanline;
		self.handle_stat_irq(interrupts);
		self.ly_lyc = self.ly == self.lyc;
	}

	pub fn tick(&mut self, interrupts: &mut Interrupts) -> bool {
		if self.enabled() {
			match self.mode() {
				PPUMode::SearchingOAM => {
					if self.current_dot == 0 {
						if self.ly == 0 {
							self.wy_match = self.wy == self.ly;
						}
						self.sprite_buffer = [None; 10];
						self.sprite_count = 0;
					}

					if self.current_dot % 2 == 0 {
						let oam_item_idx: usize = (self.current_dot as usize / 2) * 4;

						let oam_entry = OAMEntry::parse([
							self.internal_read_oam(oam_item_idx),
							self.internal_read_oam(oam_item_idx + 1),
							self.internal_read_oam(oam_item_idx + 2),
							self.internal_read_oam(oam_item_idx + 3),
						]);

						let sprite_height = self.sprite_height();

						let real_oam_y =
							oam_entry.y.wrapping_sub(16).wrapping_add(sprite_height as u8);

						if oam_entry.x > 0
							&& self.ly < real_oam_y
							&& self.ly >= oam_entry.y.wrapping_sub(16)
							&& self.sprite_count < 10
						{
							self.sprite_buffer[self.sprite_count] = Some(oam_entry);
							self.sprite_count += 1;
						}
					}

					self.current_dot += 1;
					self.total_dots += 1;

					if self.current_dot == 80 {
						self.set_mode(interrupts, PPUMode::TransferringData);
						assert_eq!(self.total_dots, 80);
					} else {
						assert!(self.current_dot < 80);
					}

					false
				}
				PPUMode::TransferringData => {
					if !self.first_frame && self.current_dot == 0 {
						assert_eq!(self.total_dots, 80);
					}
					// assert!(self.current_dot < self.dot_target);

					if self.current_dot >= 12 {
						match self.current_draw_state {
							Some(LineDrawingState::Finished) => {}
							_ => {
								self.draw_pixel();
							}
						}
					}

					self.current_dot += 1;
					self.total_dots += 1;

					// if self.current_dot == self.dot_target - 1 {
					//     self.handle_stat_irq(interrupts, true);
					// }

					if self.current_dot == self.dot_target {
						// assert_eq!(self.total_dots, match self.first_frame && self.first_line {
						//     true => self.dot_target,
						//     false => 80 + self.dot_target
						// });
						assert_eq!(self.current_draw_state, Some(LineDrawingState::Finished));
						self.set_mode(interrupts, PPUMode::HBlank);
					}

					false
				}
				PPUMode::HBlank => {
					if self.first_line && self.current_dot == 16 && self.dot_target == 0 {
						// Special case for resets
						// self.handle_stat_irq(interrupts, false);
						self.set_mode(interrupts, PPUMode::TransferringData);
					} else if self.dot_target != 0 && self.current_dot == self.dot_target {
						self.set_scanline(interrupts, self.ly + 1);

						assert_eq!(
							self.total_dots,
							match self.first_frame && self.first_line {
								true => 456 - (80 - 16),
								false => 456,
							}
						);
						self.total_dots = 0;
						self.first_line = false;

						let next_mode = match self.ly > 143 {
							true => PPUMode::VBlank,
							false => PPUMode::SearchingOAM,
						};

						self.set_mode(interrupts, next_mode);
					} else {
						self.current_dot += 1;
						self.total_dots += 1;
						if !self.first_line {
							assert_ne!(self.dot_target, 0);
						}
					}

					false
				}
				PPUMode::VBlank => {
					if self.current_dot != 0 && self.current_dot % 456 == 0 {
						if self.ly >= 153 {
							self.set_scanline(interrupts, 0);
							self.set_mode(interrupts, PPUMode::SearchingOAM);
							self.first_frame = false;
							true
						} else {
							self.set_scanline(interrupts, self.ly + 1);
							self.current_dot += 1;
							false
						}
					} else {
						assert!(self.current_dot < 4560);
						self.current_dot += 1;
						false
					}
				}
			}
		} else {
			false
		}
	}

	fn parse_tile_color(tile: &[u8], x: usize, y: usize, palette: &Palette) -> (u8, Color) {
		assert!(x < 8);
		assert!(y < 8);
		let bitshift = 7 - x;
		Color::parse_bgp_color(tile[y * 2] >> bitshift, tile[(y * 2) + 1] >> bitshift, palette)
	}

	fn clear_line_sprite_fb(&mut self, real_line_number: usize) {
		assert!(real_line_number < FB_HEIGHT as usize);
		for value in 0..256 {
			let idx = ((real_line_number * FB_WIDTH as usize) + value) * 4;
			self.sprite_framebuffer[idx] = 0;
			self.sprite_framebuffer[idx + 1] = 0;
			self.sprite_framebuffer[idx + 2] = 0;
			self.sprite_framebuffer[idx + 3] = 0;
		}
	}

	fn draw_pixel(&mut self) {
		let state = match self.current_draw_state.take() {
			Some(state) => state,
			None => {
				let scrolling_delay = self.scx % 8;
				self.dot_target += scrolling_delay as u16;
				if scrolling_delay != 0 {
					LineDrawingState::BackgroundScrolling(
						scrolling_delay as usize,
						self.scx,
						self.scy,
					)
				} else {
					self.clear_line_sprite_fb(self.ly as usize);
					LineDrawingState::BackgroundAndObjectFifo(
						self.scx,
						self.scy,
						0,
						false,
						self.lcdc & 0b1 == 0,
					)
				}
			}
		};

		match state {
			LineDrawingState::BackgroundScrolling(mut remaining_cycles, scx, scy) => {
				assert_ne!(remaining_cycles, 0);

				remaining_cycles -= 1;
				self.current_draw_state =
					Some(LineDrawingState::BackgroundScrolling(remaining_cycles, scx, scy));
				if remaining_cycles == 0 {
					self.current_draw_state = Some(LineDrawingState::BackgroundAndObjectFifo(
						scx,
						scy,
						0,
						false,
						self.lcdc & 0b1 == 0,
					));
					self.clear_line_sprite_fb(self.ly as usize);
				}
			}
			LineDrawingState::BackgroundAndObjectFifo(
				scx,
				scy,
				mut drawn_pixels,
				mut window_drawn,
				draw_only_sprites,
			) => {
				// assert_eq!(scy, self.scy);
				// assert_eq!(scx, self.scx);
				let wx_match = (drawn_pixels as usize + 7) >= self.wx as usize;
				let scrolled_y = self.ly.wrapping_add(scy) as usize;
				let scrolled_x = drawn_pixels.wrapping_add(scx) as usize;

				let (bg_color_id, bg_color) = match draw_only_sprites {
					true => (0, Color::White),
					false => {
						let tilemap_idx = scrolled_x / 8 + ((scrolled_y / 8) * 32);
						let tilemap_value = self.read_tile_map()[tilemap_idx];
						let (mut bg_color_id, mut bg_color) = Self::parse_tile_color(
							self.read_bg_win_tile(tilemap_value),
							scrolled_x % 8,
							scrolled_y % 8,
							&self.bgp,
						);

						if self.window_enabled() && wx_match && self.wy_match {
							window_drawn = true;
							let window_x = (drawn_pixels as u8).wrapping_sub(self.wx.wrapping_sub(7)) as usize;
							let window_y = self.ly.wrapping_sub(self.wy) as usize;
							let tilemap_idx = window_x / 8 + ((window_y / 8) * 32);
							let tilemap_value = self.read_window_tile_map()[tilemap_idx];
							(bg_color_id, bg_color) = Self::parse_tile_color(
								self.read_bg_win_tile(tilemap_value),
								window_x % 8,
								window_y % 8,
								&self.bgp,
							);
						}

						(bg_color_id, bg_color)
					}
				};

				let framebuffer_offset =
					((self.ly as usize * FB_WIDTH as usize) + drawn_pixels as usize) * 4;
				for (idx, byte) in bg_color.rgba().iter().enumerate() {
					self.framebuffer[framebuffer_offset + idx] = *byte;
				}

				let mut sprite_buffer = Vec::new();
				for sprite_idx in 0..self.sprite_count {
					// WARNING: Sprites are not scrolled, they have an absolute position!
					let sprite = self.sprite_buffer[sprite_idx]
						.as_ref()
						.expect("within the sprite count there should be non `None`s");

					let x_valid =
						drawn_pixels < sprite.x && drawn_pixels.wrapping_add(8) >= sprite.x;
					let y_valid = self.ly < sprite.y && self.ly.wrapping_add(16) >= sprite.y;

					if x_valid && y_valid {
						sprite_buffer.push(*sprite);
					}
				}

				sprite_buffer.sort_by(|l, r| l.x.cmp(&r.x));

				// TODO: Adjust mode length based on sprites
				for sprite in &sprite_buffer {
					let mut sprite_x_idx =
						drawn_pixels.wrapping_sub(sprite.x.wrapping_sub(8)) as usize;
					let mut sprite_y_idx = self.ly.wrapping_sub(sprite.y.wrapping_sub(16)) as usize;

					let tile_idx = match self.sprite_height() {
						SpriteHeight::Eight => sprite.tile_idx,
						SpriteHeight::Sixteen => match sprite_y_idx >= 8 {
							true => sprite.tile_idx | 1,
							false => sprite.tile_idx & 0xFE,
						},
					};

					if sprite_y_idx >= 8 {
						sprite_y_idx -= 8;
						assert!(sprite_y_idx < 8);
					}

					if sprite.y_flip() {
						sprite_y_idx = 7 - sprite_y_idx;
					}

					if sprite.x_flip() {
						sprite_x_idx = 7 - sprite_x_idx;
					}

					let color_id =
						self.read_obj_tile_colour_id(tile_idx, sprite_x_idx, sprite_y_idx);
					let palette = &self.obp[sprite.palette_number() as usize];
					let sprite_color = palette.color_from_2bit(color_id);

					if color_id != 0 && !(sprite.covered_by_bg_window() && bg_color_id != 0) {
						self.sprite_framebuffer[framebuffer_offset + 0] = sprite_color.rgba()[0];
						self.sprite_framebuffer[framebuffer_offset + 1] = sprite_color.rgba()[1];
						self.sprite_framebuffer[framebuffer_offset + 2] = sprite_color.rgba()[2];
						self.sprite_framebuffer[framebuffer_offset + 3] = sprite_color.rgba()[3];
					}
				}

				drawn_pixels += 1;
				if drawn_pixels == FB_WIDTH as u8 {
					if window_drawn {
						self.dot_target += 6;
					}
					self.current_draw_state = Some(LineDrawingState::Finished);
				} else {
					self.current_draw_state = Some(LineDrawingState::BackgroundAndObjectFifo(
						scx,
						scy,
						drawn_pixels,
						window_drawn,
						draw_only_sprites,
					));
				}
			}
			LineDrawingState::Finished => unreachable!(),
		}
	}

	pub fn window_enabled(&self) -> bool {
		((self.lcdc >> 5) & 0b1) == 1
	}

	pub fn write_fb(&self) -> Vec<u8> {
		let mut out = self.framebuffer.0.to_vec();

		for x in 0..(160 * 144) {
			let idx = x * 4;

			let (r, g, b, a) = (
				self.sprite_framebuffer[idx],
				self.sprite_framebuffer[idx + 1],
				self.sprite_framebuffer[idx + 2],
				self.sprite_framebuffer[idx + 3],
			);

			if r != 0 || g != 0 || b != 0 || a != 0 {
				out[idx] = r;
				out[idx + 1] = g;
				out[idx + 2] = b;
				out[idx + 3] = a;
			}
		}

		out
	}

	pub fn dump_fb_to_file(&self) -> String {
		let mut image = bmp::Image::new(FB_WIDTH, FB_HEIGHT);

		for y in 0..FB_HEIGHT {
			for x in 0..FB_WIDTH {
				let base = ((y as usize * FB_WIDTH as usize) + x as usize) * 4;
				image.set_pixel(
					x,
					y,
					bmp::Pixel::new(
						self.framebuffer[base],
						self.framebuffer[base + 1],
						self.framebuffer[base + 2],
					),
				);
			}
		}

		let now = chrono::Utc::now();
		std::fs::create_dir_all("./bmp").unwrap();
		let file_name = format!("./bmp/fb-{}.bmp", now.timestamp());
		image.save(file_name.as_str()).unwrap();
		file_name
	}

	pub fn dump_bg_tiles(&self) {
		let mut image = bmp::Image::new(16 * 8, 16 * 9);

		for tile_y in 0..16 {
			for tile_x in 0..16 {
				let tiledata = self.read_bg_win_tile(tile_y * 16 + tile_x);
				for row in 0..8usize {
					let base = row * 2;

					let pixels = Color::parse_bgp(tiledata[base], tiledata[base + 1], &self.bgp);

					for (x, color) in pixels.iter().enumerate() {
						let pixel = color.rgba();
						image.set_pixel(
							(tile_x as u32 * 8) + x as u32,
							tile_y as u32 * 9 + row as u32,
							bmp::Pixel::new(pixel[0], pixel[1], pixel[2]),
						);
					}
				}

				for x in 0..8 {
					let color = bmp::Pixel::new(255 / (x + 1), 255 / (x + 1), 255 / (x + 1));
					image.set_pixel((tile_x as u32 * 8) + x as u32, 9 * tile_y as u32, color);
				}
			}
		}

		let now = chrono::Utc::now();
		std::fs::create_dir_all("./bmp").unwrap();
		let file_name = format!("./bmp/bg-data-{}.bmp", now.timestamp());
		image.save(file_name.as_str()).unwrap();
	}

	pub fn read_bg_win_tile(&self, idx: u8) -> &[u8] {
		if (self.lcdc >> 4) & 0b1 == 1 {
			&self.vram[idx as usize * 16..((idx as usize + 1) * 16)]
		} else if idx < 128 {
			&self.vram[0x1000 + (idx as usize * 16)..0x1000 + ((idx as usize + 1) * 16)]
		} else {
			let adjusted_obj = idx - 128;
			&self.vram
				[0x800 + (adjusted_obj as usize * 16)..0x800 + ((adjusted_obj as usize + 1) * 16)]
		}
	}
}
