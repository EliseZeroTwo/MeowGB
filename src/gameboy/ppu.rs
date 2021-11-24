use std::ops::{Index, IndexMut};

use crate::{gameboy::Interrupts, window::FB_WIDTH};

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
	HBlank,
	VBlank,
	SearchingOAM,
	TransferringData,
}

#[derive(Debug, Clone, Copy)]
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

	pub fn parse_bgp_color(color: u8) -> Self {
		match color & 0b11 {
			0 => Self::White,
			1 => Self::LGray,
			2 => Self::DGray,
			3 => Self::Black,
			_ => unreachable!(),
		}
	}

	pub fn parse_obp_color(color: u8) -> Self {
		match color & 0b11 {
			0 => Self::Transparent,
			1 => Self::LGray,
			2 => Self::DGray,
			3 => Self::Black,
			_ => unreachable!(),
		}
	}

	pub fn parse_bgp(mut bgp: u8) -> [Self; 4] {
		let mut out = [Self::White, Self::White, Self::White, Self::White];
		for color in &mut out {
			*color = Self::parse_bgp_color(bgp);
			bgp >>= 2;
		}
		out.reverse();
		out
	}

	pub fn parse_obp(mut obp: u8) -> [Self; 4] {
		let mut out = [Self::Transparent, Self::Transparent, Self::Transparent, Self::Transparent];
		for color in &mut out {
			*color = Self::parse_obp_color(obp);
			obp >>= 2;
		}
		out.reverse();
		out
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
		for pixel_idx in 0..FB_WIDTH as u8 {
			let scrolled_x = pixel_idx.overflowing_add(self.scx).0 as usize;
			let scrolled_y = self.ly.overflowing_add(self.scy).0 as usize;
			let tilemap_idx = scrolled_x / 8 + ((scrolled_y as usize / 8) * 32);
			let tilemap_value = self.read_tile_map()[tilemap_idx];

			let color = Self::parse_tile_color(
				self.read_tile(tilemap_value),
				scrolled_x % 8,
				scrolled_y % 8,
			);
			let dest_idx_base = ((self.scy as usize * FB_WIDTH as usize) + pixel_idx as usize) * 4;
			for (idx, byte) in color.rgba().iter().enumerate() {
				self.framebuffer[dest_idx_base + idx] = *byte;
			}
		}
	}

	fn parse_tile_color(tile: &[u8], x: usize, y: usize) -> Color {
		assert!(x < 8);
		if x < 4 {
			let bitshift = 6 - x * 2;
			Color::parse_bgp_color(tile[y * 2] >> bitshift)
		} else {
			let x = x - 4;
			let bitshift = 6 - x * 2;
			Color::parse_bgp_color(tile[(y * 2) + 1] >> bitshift)
		}
	}

	fn set_mode(&mut self, interrupts: &mut Interrupts, mode: PPUMode) {
		log::debug!("PPU switching mode to {:?} @ {}", mode, self.cycle_counter);
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

	pub fn tick(&mut self, interrupts: &mut Interrupts) -> bool {
		let res = match self.mode() {
			PPUMode::HBlank => {
				if self.cycle_counter >= 120 {
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
				if self.cycle_counter % 506 == 0 {
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
				if self.cycle_counter >= 80 {
					self.set_mode(interrupts, PPUMode::TransferringData);
				}
				false
			}
			PPUMode::TransferringData => {
				if self.cycle_counter >= 170 {
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
			PPUMode::HBlank | PPUMode::VBlank => self.oam[decoded_address as usize],
			PPUMode::SearchingOAM | PPUMode::TransferringData => 0xFF,
		}
	}

	pub fn dma_write_oam(&mut self, offset: u8, value: u8) {
		self.oam[offset as usize] = value;
	}

	pub fn cpu_write_oam(&mut self, address: u16, value: u8) {
		let decoded_address = address - 0xFE00;
		match self.mode() {
			PPUMode::HBlank | PPUMode::VBlank => self.oam[decoded_address as usize] = value,
			_ => {}
		}
	}

	pub fn dma_read_vram(&mut self, offset: u8) -> u8 {
		self.vram[offset as usize]
	}

	pub fn cpu_read_vram(&self, address: u16) -> u8 {
		let decoded_address = address - 0x8000;
		match self.mode() {
			PPUMode::HBlank | PPUMode::VBlank | PPUMode::SearchingOAM => {
				self.vram[decoded_address as usize]
			}
			PPUMode::TransferringData => 0xFF,
		}
	}

	pub fn cpu_write_vram(&mut self, address: u16, value: u8) {
		let decoded_address = address - 0x8000;
		match self.mode() {
			PPUMode::HBlank | PPUMode::VBlank | PPUMode::SearchingOAM => {
				self.vram[decoded_address as usize] = value
			}
			_ => {}
		}
	}

	pub fn cpu_write_stat(&mut self, value: u8) {
		self.stat = value & 0b0111_1000;
	}

	pub fn read_tile(&self, obj: u8) -> &[u8] {
		if (self.lcdc >> 4) & 0b1 == 1 {
			&self.vram[obj as usize * 16..((obj as usize + 1) * 16)]
		} else if obj < 128 {
			&self.vram[0x1000 + (obj as usize * 16)..0x1000 + ((obj as usize + 1) * 16)]
		} else {
			let adjusted_obj = obj - 128;
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
