/// Provides an [egui] based overlay for debugigng the emulator whilst it is
/// running
use egui::{ClippedPrimitive, Context, Grid, TexturesDelta, RichText, Color32};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use meowgb_core::gameboy::serial::SerialWriter;
use pixels::{wgpu, PixelsContext};
use winit::{event_loop::EventLoopWindowTarget, window::Window};

use super::events::{EmulatorDebugEvent, EmulatorWindowEvent};
use crate::WrappedGameboy;

pub(crate) struct Framework {
	egui_ctx: Context,
	egui_state: egui_winit::State,
	screen_descriptor: ScreenDescriptor,
	renderer: Renderer,
	paint_jobs: Vec<ClippedPrimitive>,
	textures: TexturesDelta,
	pub gui: Gui,
}

#[derive(Debug, Clone, Copy)]
pub struct GuiWindowState {
	pub window_open: bool,
	pub register_window_open: bool,
	pub ppu_register_window_open: bool,
	pub debugger_window_open: bool,
	pub wram_window_open: bool,
	pub oam_window_open: bool,
	pub hram_window_open: bool,
	pub dma_window_open: bool,
}

impl GuiWindowState {
	pub fn close_all(&mut self) {
		self.ppu_register_window_open = false;
		self.register_window_open = false;
		self.window_open = false;
		self.debugger_window_open = false;
		self.wram_window_open = false;
		self.oam_window_open = false;
		self.hram_window_open = false;
		self.dma_window_open = false;
	}

	pub fn any_open(&self) -> bool {
		self.window_open
			|| self.register_window_open
			|| self.ppu_register_window_open
			|| self.debugger_window_open
			|| self.wram_window_open
			|| self.oam_window_open
			|| self.hram_window_open
			|| self.dma_window_open
	}
}

pub struct Gui {
	pub state: GuiWindowState,
	pub state_restore: Option<GuiWindowState>,
	pub registers: meowgb_core::gameboy::cpu::Registers,
	pub ppu_registers: meowgb_core::gameboy::ppu::PpuRegisters,
	pub wram: [u8; 0x2000],
	pub hram: [u8; 0xAF],
	// pub vram: [u8; 0x2000],
	pub oam: [u8; 0xA0],
	pub bp_string: String,
	pub bp_read_checkbox: bool,
	pub bp_write_checkbox: bool,
	pub bp_execute_checkbox: bool,
	pub is_debugging: bool,
	pub breakpoints: [[bool; 3]; 0x10000],
	pub sender: std::sync::mpsc::Sender<EmulatorWindowEvent>,
	pub dma: meowgb_core::gameboy::dma::DmaState,
}

impl Framework {
	pub(crate) fn new<T>(
		event_loop: &EventLoopWindowTarget<T>,
		width: u32,
		height: u32,
		scale_factor: f32,
		pixels: &pixels::Pixels,
		gameboy: &WrappedGameboy<impl SerialWriter>,
		sender: std::sync::mpsc::Sender<EmulatorWindowEvent>,
	) -> Self {
		let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

		let egui_ctx = Context::default();
		let mut egui_state = egui_winit::State::new(event_loop);
		egui_state.set_max_texture_side(max_texture_size);
		egui_state.set_pixels_per_point(scale_factor);
		let screen_descriptor =
			ScreenDescriptor { size_in_pixels: [width, height], pixels_per_point: scale_factor };
		let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
		let textures = TexturesDelta::default();
		let gui = Gui::new(gameboy, sender);

		Self {
			egui_ctx,
			egui_state,
			screen_descriptor,
			renderer,
			paint_jobs: Vec::new(),
			textures,
			gui,
		}
	}

	pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) -> bool {
		self.egui_state.on_event(&self.egui_ctx, event).repaint
	}

	pub(crate) fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			self.screen_descriptor.size_in_pixels = [width, height];
		}
	}

	pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
		self.screen_descriptor.pixels_per_point = scale_factor as f32;
	}

	pub(crate) fn prepare(&mut self, window: &Window, gameboy: &WrappedGameboy<impl SerialWriter>) {
		self.gui.registers = gameboy.gameboy.registers;
		self.gui.ppu_registers = gameboy.gameboy.ppu.registers;
		self.gui.is_debugging = gameboy.debugging;
		self.gui.oam = gameboy.gameboy.ppu.oam;
		self.gui.hram = gameboy.gameboy.memory.hram;
		self.gui.wram = gameboy.gameboy.memory.wram;
		self.gui.dma = gameboy.gameboy.dma;

		// Run the egui frame and create all paint jobs to prepare for rendering.
		let raw_input = self.egui_state.take_egui_input(window);
		let output = self.egui_ctx.run(raw_input, |egui_ctx| {
			// Draw the demo application.
			self.gui.ui(egui_ctx);
		});

		self.textures.append(output.textures_delta);
		self.egui_state.handle_platform_output(window, &self.egui_ctx, output.platform_output);
		self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
	}

	pub(crate) fn render(
		&mut self,
		encoder: &mut wgpu::CommandEncoder,
		render_target: &wgpu::TextureView,
		context: &PixelsContext,
	) {
		for (id, image_delta) in &self.textures.set {
			self.renderer.update_texture(&context.device, &context.queue, *id, image_delta);
		}
		self.renderer.update_buffers(
			&context.device,
			&context.queue,
			encoder,
			&self.paint_jobs,
			&self.screen_descriptor,
		);

		{
			let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("egui"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: render_target,
					resolve_target: None,
					ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: true },
				})],
				depth_stencil_attachment: None,
			});

			self.renderer.render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
		}

		let textures = std::mem::take(&mut self.textures);
		for id in &textures.free {
			self.renderer.free_texture(id);
		}
	}
}

impl Gui {
	fn new(
		gameboy: &WrappedGameboy<impl SerialWriter>,
		sender: std::sync::mpsc::Sender<EmulatorWindowEvent>,
	) -> Self {
		Self {
			state: GuiWindowState {
				window_open: gameboy.debugging,
				register_window_open: false,
				ppu_register_window_open: false,
				debugger_window_open: gameboy.debugging,
				wram_window_open: false,
				oam_window_open: false,
				hram_window_open: false,
    			dma_window_open: false,
			},
			state_restore: None,
			registers: gameboy.gameboy.registers,
			ppu_registers: gameboy.gameboy.ppu.registers,
			bp_string: String::with_capacity(16),
			breakpoints: [[false, false, false]; 0x10000],
			bp_read_checkbox: false,
			bp_write_checkbox: false,
			bp_execute_checkbox: false,
			sender,
			is_debugging: gameboy.debugging,
			wram: gameboy.gameboy.memory.wram,
			hram: gameboy.gameboy.memory.hram,
			oam: gameboy.gameboy.ppu.oam,
			dma: gameboy.gameboy.dma
		}
	}

	fn ui(&mut self, ctx: &Context) {
		egui::Window::new("MeowGB Debugger").open(&mut self.state.window_open).show(ctx, |ui| {
			if ui.button("Toggle Debugger Window").clicked() {
				self.state.debugger_window_open = !self.state.debugger_window_open;
			}

			if ui.button("Toggle Register Window").clicked() {
				self.state.register_window_open = !self.state.register_window_open;
			}

			if ui.button("Toggle PPU Window").clicked() {
				self.state.ppu_register_window_open = !self.state.ppu_register_window_open;
			}

			if ui.button("Toggle BG Tiles Window").clicked() {
				self.state.ppu_register_window_open = !self.state.ppu_register_window_open;
			}

			if ui.button("Toggle WRAM Window").clicked() {
				self.state.wram_window_open = !self.state.wram_window_open;
			}

			if ui.button("Toggle HRAM Window").clicked() {
				self.state.hram_window_open = !self.state.hram_window_open;
			}

			if ui.button("Toggle OAM Window").clicked() {
				self.state.oam_window_open = !self.state.oam_window_open;
			}
			
			if ui.button("Toggle DMA Window").clicked() {
				self.state.dma_window_open = !self.state.dma_window_open;
			}
		});

		egui::Window::new("Register State").open(&mut self.state.register_window_open).show(
			ctx,
			|ui| {
				ui.label(format!("AF: {:04X}", self.registers.get_af()));
				ui.label(format!("BC: {:04X}", self.registers.get_bc()));
				ui.label(format!("DE: {:04X}", self.registers.get_de()));
				ui.label(format!("HL: {:04X}", self.registers.get_hl()));
				ui.label(format!("SP: {:04X}", self.registers.get_sp()));
				ui.label(format!("PC: {:04X}", self.registers.pc));
			},
		);

		egui::Window::new("Debugger").open(&mut self.state.debugger_window_open).show(ctx, |ui| {
			if ui.button("Step").clicked() {
				let _ = self.sender.send(EmulatorWindowEvent::Debug(EmulatorDebugEvent::Step));
			}
			if ui.button("Continue").clicked() {
				let _ = self.sender.send(EmulatorWindowEvent::Debug(EmulatorDebugEvent::Continue));
			}
			ui.label("Toggle Breakpoint");
			ui.text_edit_singleline(&mut self.bp_string);
			self.bp_string.retain(|x| x.is_ascii_hexdigit());
			if let Some((fourth_index, _)) = self.bp_string.char_indices().nth(4) {
				self.bp_string.truncate(fourth_index);
			}
			Grid::new("debugger_bp_select_grid").show(ui, |ui| {
				let address = u16::from_str_radix(self.bp_string.as_str(), 16).unwrap_or_default();
				ui.label(format!("({:#X}) ", address));
				let [read, write, execute] = &mut self.breakpoints[address as usize];
				let mut changed = ui.checkbox(read, "Read").clicked();
				changed |= ui.checkbox(write, "Write").clicked();
				changed |= ui.checkbox(execute, "Execute").clicked();
				if changed {
					let _ = self.sender.send(EmulatorWindowEvent::Debug(
						EmulatorDebugEvent::ToggleBreakpoint(
							address,
							self.breakpoints[address as usize],
						),
					));
				}
			});

			Grid::new("debugger_bp_list_grid").show(ui, |ui| {
				ui.heading("Enabled BPs");
				ui.end_row();
				for (idx, [read, write, execute]) in self.breakpoints.iter_mut().enumerate() {
					if *read || *write || *execute {
						ui.label(format!("{:#X}: ", idx));
						let mut changed = ui.checkbox(read, "Read").clicked();
						changed |= ui.checkbox(write, "Write").clicked();
						changed |= ui.checkbox(execute, "Execute").clicked();
						if changed {
							let _ = self.sender.send(EmulatorWindowEvent::Debug(
								EmulatorDebugEvent::ToggleBreakpoint(
									idx as u16,
									[*read, *write, *execute],
								),
							));
						}
						ui.end_row();
					}
				}
			});
		});

		egui::Window::new("PPU State").open(&mut self.state.ppu_register_window_open).show(
			ctx,
			|ui| {
				ui.label(format!("Mode: {:?}", self.ppu_registers.mode));
				ui.label(format!("LCDC: {:02X}", self.ppu_registers.lcdc));
				ui.label(format!(
					"Stat: {:02X}",
					(1 << 7)
						| self.ppu_registers.stat_flags.flag_bits()
						| ((match (self.ppu_registers.lcdc >> 7) == 1 {
							true => self.ppu_registers.ly == self.ppu_registers.lyc,
							false => self.ppu_registers.ly_lyc,
						} as u8) << 2) | self.ppu_registers.mode.mode_flag()
				));
				ui.label(format!("SCY: {:02X}", self.ppu_registers.scy));
				ui.label(format!("SCX: {:02X}", self.ppu_registers.scx));
				ui.label(format!("LY: {:02X}", self.ppu_registers.ly));
				ui.label(format!("LYC: {:02X}", self.ppu_registers.lyc));
				ui.label(format!("WY: {:02X}", self.ppu_registers.wy));
				ui.label(format!("WX: {:02X}", self.ppu_registers.wx));
			},
		);

		egui::Window::new("WRAM").vscroll(true).open(&mut self.state.wram_window_open).show(ctx, |ui| {
			egui::Grid::new("memory_ov_wram").show(ui, |ui| {
				ui.monospace("      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
				for mem_row_idx in 0xC00..(0xE00) {
					let row_base = (mem_row_idx * 0x10) - 0xC000;
					ui.end_row();
					ui.monospace(format!("{:X}: {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", mem_row_idx * 0x10, self.wram[row_base + 0], self.wram[row_base + 1], self.wram[row_base + 2], self.wram[row_base + 3], self.wram[row_base + 4], self.wram[row_base + 5], self.wram[row_base + 6], self.wram[row_base + 7], self.wram[row_base + 8], self.wram[row_base + 9], self.wram[row_base + 10], self.wram[row_base + 11], self.wram[row_base + 12], self.wram[row_base + 13], self.wram[row_base + 14], self.wram[row_base + 15]));
				}
			});
		});

		egui::Window::new("DMA").vscroll(true).open(&mut self.state.dma_window_open).show(ctx, |ui| {
			if let Some(bus) = self.dma.in_progress() {
				ui.heading(RichText::new(format!("Active ({:#?} Bus)", bus)).color(Color32::LIGHT_GREEN));
			} else {
				ui.heading(RichText::new("Inactive").color(Color32::LIGHT_RED));
			}

			let offset = (0xA0 - self.dma.remaining_cycles) as u16;
			ui.label(format!("Read Address:  {:#04X}", ((self.dma.base as u16) << 8) | offset));
			ui.label(format!("Write Address: {:#04X}", 0xFE00 | offset));
			ui.label(format!("Base: {:#04X}", (self.dma.base as u16) << 8));
			ui.label(format!("Remaining Bytes: {:#02X}", self.dma.remaining_cycles));
		});

		egui::Window::new("HRAM").vscroll(true).open(&mut self.state.hram_window_open).show(ctx, |ui| {
			egui::Grid::new("memory_ov_hram").show(ui, |ui| {
				ui.label("ROW: 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
				for mem_row_idx in 0x0..0xA {
					let row_base = mem_row_idx * 0x10;
					ui.end_row();
					ui.monospace(format!("{:X}: {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", 0xFF80 + row_base, self.hram[row_base + 0], self.hram[row_base + 1], self.hram[row_base + 2], self.hram[row_base + 3], self.hram[row_base + 4], self.hram[row_base + 5], self.hram[row_base + 6], self.hram[row_base + 7], self.hram[row_base + 8], self.hram[row_base + 9], self.hram[row_base + 10], self.hram[row_base + 11], self.hram[row_base + 12], self.hram[row_base + 13], self.hram[row_base + 14], self.hram[row_base + 15]));
				}

				let row_base = 0xA0;
				ui.end_row();
				ui.monospace(format!("FFF0: {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} ??", self.hram[row_base + 0], self.hram[row_base + 1], self.hram[row_base + 2], self.hram[row_base + 3], self.hram[row_base + 4], self.hram[row_base + 5], self.hram[row_base + 6], self.hram[row_base + 7], self.hram[row_base + 8], self.hram[row_base + 9], self.hram[row_base + 10], self.hram[row_base + 11], self.hram[row_base + 12], self.hram[row_base + 13], self.hram[row_base + 14]));
			});
		});

		egui::Window::new("OAM").vscroll(true).open(&mut self.state.oam_window_open).show(ctx, |ui| {
			egui::Grid::new("memory_ov_oam").show(ui, |ui| {
				ui.label("ROW: 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
				for mem_row_idx in 0xC00..(0xC0A) {
					let row_base = (mem_row_idx * 0x10) - 0xC000;
					ui.end_row();
					ui.label(format!("{:X}: {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}", mem_row_idx * 0x10, self.hram[row_base + 0], self.hram[row_base + 1], self.hram[row_base + 2], self.hram[row_base + 3], self.hram[row_base + 4], self.hram[row_base + 5], self.hram[row_base + 6], self.hram[row_base + 7], self.hram[row_base + 8], self.hram[row_base + 9], self.hram[row_base + 10], self.hram[row_base + 11], self.hram[row_base + 12], self.hram[row_base + 13], self.hram[row_base + 14], self.hram[row_base + 15]));
				}

				let row_base = 0xA0;
				ui.end_row();
				ui.label(format!("CF00: {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} ??", self.hram[row_base + 0], self.hram[row_base + 1], self.hram[row_base + 2], self.hram[row_base + 3], self.hram[row_base + 4], self.hram[row_base + 5], self.hram[row_base + 6], self.hram[row_base + 7], self.hram[row_base + 8], self.hram[row_base + 9], self.hram[row_base + 10], self.hram[row_base + 11], self.hram[row_base + 12], self.hram[row_base + 13], self.hram[row_base + 14]));
			});
		});
	}
}
