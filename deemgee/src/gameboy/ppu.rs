use std::ops::{Index, IndexMut};

use bmp::{Image, Pixel};

use crate::{
	gameboy::Interrupts,
	window::{FB_HEIGHT, FB_WIDTH},
};

pub struct WrappedBuffer<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> Index<usize> for WrappedBuffer<SIZE> {
	type Output = u8;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index % SIZE]
	}
}

impl<const SIZE: usize> IndexMut<usize> for WrappedBuffer<SIZE> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index % SIZE]
	}
}

impl<const SIZE: usize> WrappedBuffer<SIZE> {
	pub fn empty() -> Self {
		Self([0; SIZE])
	}
}

#[derive(Debug, PartialEq, Eq)]
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
	pub fn rgba(self) -> &'static [u8; 4] {
		match self {
			Color::White => &[0x7F, 0x86, 0x0F, 0xFF],
			Color::LGray => &[0x57, 0x7c, 0x44, 0xFF],
			Color::DGray => &[0x36, 0x5d, 0x48, 0xFF],
			Color::Black => &[0x2a, 0x45, 0x3b, 0xFF],
			Color::Transparent => &[0x00, 0x00, 0x00, 0x00],
		}
	}

	pub fn parse_bgp_color(low: u8, high: u8) -> Self {
		let color = ((high & 0b1) << 1) | low & 0b1;
		match color & 0b11 {
			0 => Self::White,
			1 => Self::LGray,
			2 => Self::DGray,
			3 => Self::Black,
			_ => unreachable!(),
		}
	}

	pub fn parse_obp_color(low: u8, high: u8) -> Self {
		let color = ((high & 0b1) << 1) | low & 0b1;
		match color & 0b11 {
			0 => Self::Transparent,
			1 => Self::LGray,
			2 => Self::DGray,
			3 => Self::Black,
			_ => unreachable!(),
		}
	}

	pub fn parse_bgp(mut bgp_low: u8, mut bgp_high: u8) -> [Self; 8] {
		let mut out = [Self::White; 8];
		for color in &mut out {
			*color = Self::parse_bgp_color(bgp_low, bgp_high);
			bgp_low >>= 1;
			bgp_high >>= 1;
		}
		out.reverse();
		out
	}
}

#[derive(Debug)]
pub struct OAMEntry {
	pub y: u8,
	pub x: u8,
	pub tile_idx: u8,
	pub flags: u8,
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

	#[allow(unused)]
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

	pub fn from_mode_flag(value: u8) -> Self {
		match value & 0b11 {
			0 => Self::HBlank,
			1 => Self::VBlank,
			2 => Self::SearchingOAM,
			3 => Self::TransferringData,
			_ => unreachable!(),
		}
	}
}

pub struct Ppu {
	pub lcdc: u8,
	pub stat: u8,
	pub scy: u8,
	pub scx: u8,
	pub ly: u8,
	pub lyc: u8,
	pub wy: u8,
	pub wx: u8,
	pub vram: [u8; 0x2000],
	pub oam: [u8; 0xA0],
	pub cycle_counter: u16,
	pub bgp: u8,
	pub obp: u8,

	pub framebuffer: WrappedBuffer<{ 160 * 144 * 4 }>,
	pub sprite_framebuffer: WrappedBuffer<{ 160 * 144 * 4 }>,
}

impl Ppu {
	pub fn new() -> Self {
		Self {
			lcdc: 0b1000_0000,
			stat: 0b0000_0010,
			scy: 0,
			scx: 0,
			ly: 0,
			lyc: 0,
			wy: 0,
			wx: 0,
			vram: [0; 0x2000],
			oam: [0; 0xA0],
			cycle_counter: 0,
			framebuffer: WrappedBuffer::empty(),
			sprite_framebuffer: WrappedBuffer::empty(),
			bgp: 0,
			obp: 0,
		}
	}

	pub fn dump_fb(&self) -> String {
		let mut image = Image::new(FB_WIDTH, FB_HEIGHT);

		for y in 0..FB_HEIGHT {
			for x in 0..FB_WIDTH {
				let base = ((y as usize * FB_WIDTH as usize) + x as usize) * 4;
				image.set_pixel(
					x,
					y,
					Pixel::new(
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
		let mut image = Image::new(16 * 8, 16 * 9);

		for tile_y in 0..16 {
			for tile_x in 0..16 {
				let tiledata = self.read_bg_win_tile(tile_y * 16 + tile_x);
				for row in 0..8usize {
					let base = row * 2;

					let pixels = Color::parse_bgp(tiledata[base], tiledata[base + 1]);

					for (x, color) in pixels.iter().enumerate() {
						let pixel = color.rgba();
						image.set_pixel(
							(tile_x as u32 * 8) + x as u32,
							tile_y as u32 * 9 + row as u32,
							Pixel::new(pixel[0], pixel[1], pixel[2]),
						);
					}
				}

				for x in 0..8 {
					let color = Pixel::new(255 / (x + 1), 255 / (x + 1), 255 / (x + 1));
					image.set_pixel((tile_x as u32 * 8) + x as u32, 9 * tile_y as u32, color);
				}
			}
		}

		let now = chrono::Utc::now();
		std::fs::create_dir_all("./bmp").unwrap();
		let file_name = format!("./bmp/bg-data-{}.bmp", now.timestamp());
		image.save(file_name.as_str()).unwrap();
	}

	fn set_scanline(&mut self, interrupts: &mut Interrupts, scanline: u8) {
		self.ly = scanline;

		self.stat &= !(1 << 2);
		if self.ly == self.lyc {
			self.stat |= 1 << 2;

			if (self.stat >> 6) & 0b1 == 1 {
				interrupts.write_if_lcd_stat(true);
			}
		} else {
			self.stat &= !(1 << 2);
		}
	}

	fn draw_line(&mut self) {
		let scrolled_y = self.ly.overflowing_add(self.scy).0 as usize;
		for pixel_idx in 0..FB_WIDTH as u8 {
			let scrolled_x = pixel_idx.overflowing_add(self.scx).0 as usize;

			// BG
			let tilemap_idx = scrolled_x / 8 + ((scrolled_y as usize / 8) * 32);
			let tilemap_value = self.read_tile_map()[tilemap_idx];
			let color = Self::parse_tile_color(
				self.read_bg_win_tile(tilemap_value),
				scrolled_x % 8,
				scrolled_y % 8,
			);
			let dest_idx_base = ((self.ly as usize * FB_WIDTH as usize) + pixel_idx as usize) * 4;
			for (idx, byte) in color.rgba().iter().enumerate() {
				self.framebuffer[dest_idx_base + idx] = *byte;
			}
		}

		// Sprite
		let mut found_sprites = 0;
		let mut sprite_line = [0u8; 256 * 4];
		let sprite_height = if (self.lcdc >> 2) & 0b1 == 1 { 16 } else { 8 };

		for x in 0..40 {
			if found_sprites >= 10 {
				break;
			}

			let oam_offset = x * 4;
			let entry = OAMEntry::parse([
				self.oam[oam_offset],
				self.oam[oam_offset + 1],
				self.oam[oam_offset + 2],
				self.oam[oam_offset + 3],
			]);

			let mut base = entry.y.overflowing_sub(sprite_height).0;
			let mut in_range = None;

			for x in 0..sprite_height {
				if base as usize == scrolled_y {
					in_range = Some(x);
					found_sprites += 1;
					break;
				}
				base = base.overflowing_add(1).0;
			}

			if let Some(mut tile_y_idx) = in_range {
				let is_second_tile = tile_y_idx >= 8;

				if is_second_tile {
					tile_y_idx -= 8;
				}

				if entry.y_flip() {
					tile_y_idx = 7 - tile_y_idx;
				}

				let tile_idx =
					if is_second_tile { entry.tile_idx | 1 } else { entry.tile_idx & 0xFE };

				for x in 0..8 {
					let fb_x = entry.x.overflowing_sub(7 - x).0;

					let sprite_line_base = fb_x as usize * 4;

					let tile_x_idx = if entry.x_flip() { 7 - x } else { x };

					let color = Self::parse_sprite_tile_color(
						self.read_obj_tile(tile_idx),
						tile_x_idx as usize,
						tile_y_idx as usize,
					);

					let bg_fb_idx = (((self.ly as usize + self.scy as usize) * FB_WIDTH as usize)
						+ fb_x as usize) * 4;
					let ok_to_draw = if entry.covered_by_bg_window() {
						let rgba = [
							self.framebuffer[bg_fb_idx],
							self.framebuffer[bg_fb_idx + 1],
							self.framebuffer[bg_fb_idx + 2],
							self.framebuffer[bg_fb_idx + 3],
						];
						&rgba != Color::Black.rgba()
					} else {
						true
					};

					if ok_to_draw {
						for (idx, byte) in color.rgba().iter().enumerate() {
							sprite_line[sprite_line_base + idx] = *byte;
						}
					}
				}
			}
		}

		for x in (self.scx as usize)..(self.scx as usize + FB_WIDTH as usize) {
			let x = x % FB_WIDTH as usize;

			let base = x * 4;

			self.sprite_framebuffer[base] = sprite_line[x];
			self.sprite_framebuffer[base + 1] = sprite_line[x + 1];
			self.sprite_framebuffer[base + 2] = sprite_line[x + 2];
			self.sprite_framebuffer[base + 3] = sprite_line[x + 3];
		}
	}

	fn parse_sprite_tile_color(tile: &[u8], x: usize, y: usize) -> Color {
		assert!(x < 8);
		assert!(y < 8);
		let bitshift = 7 - x;
		Color::parse_obp_color(tile[y * 2] >> bitshift, tile[(y * 2) + 1] >> bitshift)
	}

	fn parse_tile_color(tile: &[u8], x: usize, y: usize) -> Color {
		assert!(x < 8);
		assert!(y < 8);
		let bitshift = 7 - x;
		Color::parse_bgp_color(tile[y * 2] >> bitshift, tile[(y * 2) + 1] >> bitshift)
	}

	fn set_mode(&mut self, interrupts: &mut Interrupts, mode: PPUMode) {
		log::trace!("PPU switching mode to {:?} @ {}", mode, self.cycle_counter);
		self.stat &= !0b11;
		self.stat |= mode.mode_flag();
		self.cycle_counter = 0;

		let offset = match mode {
			PPUMode::HBlank => 3,
			PPUMode::VBlank => 4,
			PPUMode::SearchingOAM => 5,
			_ => return,
		};

		if (self.stat >> offset) & 0b1 == 1 {
			interrupts.write_if_lcd_stat(true);
		}

		if mode == PPUMode::VBlank {
			interrupts.write_if_vblank(true);
		}
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

	/// One line should be 456 ticks (but we effectively emulate 228 ticks)
	pub fn tick(&mut self, interrupts: &mut Interrupts) -> bool {
		let res = match self.mode() {
			PPUMode::HBlank => {
				if self.cycle_counter == (102 / 2) {
					self.set_scanline(interrupts, self.ly + 1);

					let next_mode = match self.ly > 143 {
						true => PPUMode::VBlank,
						false => PPUMode::SearchingOAM,
					};
					self.set_mode(interrupts, next_mode);
				}
				false
			}
			PPUMode::VBlank => {
				if self.cycle_counter % (228 / 2) == 0 {
					if self.ly >= 153 {
						self.set_scanline(interrupts, 0);
						self.set_mode(interrupts, PPUMode::SearchingOAM);
						true
					} else {
						self.set_scanline(interrupts, self.ly + 1);
						false
					}
				} else {
					false
				}
			}
			PPUMode::SearchingOAM => {
				if self.cycle_counter == (40 / 2) {
					self.set_mode(interrupts, PPUMode::TransferringData);
				}
				false
			}
			PPUMode::TransferringData => {
				if self.cycle_counter == (86 / 2) {
					self.draw_line();
					self.set_mode(interrupts, PPUMode::HBlank);
				}
				false
			}
		};

		self.cycle_counter += 1;

		res
	}

	pub fn mode(&self) -> PPUMode {
		PPUMode::from_mode_flag(self.stat)
	}

	pub fn cpu_read_oam(&self, address: u16) -> u8 {
		let decoded_address = address - 0xFE00;
		match self.mode() {
			PPUMode::SearchingOAM
			| PPUMode::TransferringData
			| PPUMode::HBlank
			| PPUMode::VBlank => self.oam[decoded_address as usize],
			//_ => 0xFF,
		}
	}

	pub fn dma_write_oam(&mut self, offset: u8, value: u8) {
		self.oam[offset as usize] = value;
	}

	pub fn cpu_write_oam(&mut self, address: u16, value: u8) {
		let decoded_address = address - 0xFE00;
		match self.mode() {
			PPUMode::SearchingOAM
			| PPUMode::TransferringData
			| PPUMode::HBlank
			| PPUMode::VBlank => self.oam[decoded_address as usize] = value,
			// _ => {}
		}
	}

	pub fn dma_read_vram(&mut self, offset: u8) -> u8 {
		self.vram[offset as usize]
	}

	pub fn cpu_read_vram(&self, address: u16) -> u8 {
		let decoded_address = address - 0x8000;
		match self.mode() {
			PPUMode::TransferringData
			| PPUMode::HBlank
			| PPUMode::VBlank
			| PPUMode::SearchingOAM => self.vram[decoded_address as usize], /* PPUMode::TransferringData => 0xFF, */
		}
	}

	pub fn cpu_write_vram(&mut self, address: u16, value: u8) {
		let decoded_address = address - 0x8000;
		match self.mode() {
			PPUMode::TransferringData
			| PPUMode::HBlank
			| PPUMode::VBlank
			| PPUMode::SearchingOAM => self.vram[decoded_address as usize] = value, //_ => {}
		}
	}

	pub fn cpu_write_stat(&mut self, value: u8) {
		self.stat = value & 0b0111_1000;
	}

	pub fn read_obj_tile(&self, idx: u8) -> &[u8] {
		&self.vram[idx as usize * 16..((idx as usize + 1) * 16)]
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

	pub fn read_tile_map(&self) -> &[u8] {
		match (self.lcdc >> 3) & 0b1 == 1 {
			true => &self.vram[0x1C00..=0x1FFF],
			false => &self.vram[0x1800..=0x1BFF],
		}
	}
}
