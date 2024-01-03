/// Provides an [egui] based overlay for debugigng the emulator whilst it is
/// running
use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use meowgb_core::gameboy::{serial::SerialWriter, Gameboy};
use pixels::{wgpu, PixelsContext};
use winit::{event_loop::EventLoopWindowTarget, window::Window};

pub(crate) struct Framework {
	egui_ctx: Context,
	egui_state: egui_winit::State,
	screen_descriptor: ScreenDescriptor,
	renderer: Renderer,
	paint_jobs: Vec<ClippedPrimitive>,
	textures: TexturesDelta,
	pub gui: Gui,
}

pub struct Gui {
	pub window_open: bool,
	pub register_window_open: bool,
	pub ppu_register_window_open: bool,

	pub registers: meowgb_core::gameboy::cpu::Registers,
	pub ppu_registers: meowgb_core::gameboy::ppu::PpuRegisters,
}

impl Framework {
	pub(crate) fn new<T>(
		event_loop: &EventLoopWindowTarget<T>,
		width: u32,
		height: u32,
		scale_factor: f32,
		pixels: &pixels::Pixels,
		gameboy: &Gameboy<impl SerialWriter>,
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
		let gui = Gui::new(gameboy);

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

	pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
		let _ = self.egui_state.on_event(&self.egui_ctx, event);
	}

	pub(crate) fn resize(&mut self, width: u32, height: u32) {
		if width > 0 && height > 0 {
			self.screen_descriptor.size_in_pixels = [width, height];
		}
	}

	pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
		self.screen_descriptor.pixels_per_point = scale_factor as f32;
	}

	pub(crate) fn prepare(&mut self, window: &Window, gameboy: &Gameboy<impl SerialWriter>) {
		self.gui.registers = gameboy.registers;
		self.gui.ppu_registers = gameboy.ppu.registers;

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
	fn new(gameboy: &Gameboy<impl SerialWriter>) -> Self {
		Self {
			window_open: false,
			register_window_open: false,
			ppu_register_window_open: false,
			registers: gameboy.registers,
			ppu_registers: gameboy.ppu.registers,
		}
	}

	fn ui(&mut self, ctx: &Context) {
		egui::Window::new("MeowGB Debugger").open(&mut self.window_open).show(ctx, |ui| {
			if ui.button("Toggle Register Window").clicked() {
				self.register_window_open = !self.register_window_open;
			}

			if ui.button("Toggle PPU Window").clicked() {
				self.ppu_register_window_open = !self.ppu_register_window_open;
			}

			if ui.button("Toggle OAM Window").clicked() {
				self.ppu_register_window_open = !self.ppu_register_window_open;
			}

			if ui.button("Toggle BG Tiles Window").clicked() {
				self.ppu_register_window_open = !self.ppu_register_window_open;
			}
		});

		egui::Window::new("Register State").open(&mut self.register_window_open).show(ctx, |ui| {
			ui.label(format!("AF: {:04X}", self.registers.get_af()));
			ui.label(format!("BC: {:04X}", self.registers.get_bc()));
			ui.label(format!("DE: {:04X}", self.registers.get_de()));
			ui.label(format!("HL: {:04X}", self.registers.get_hl()));
			ui.label(format!("SP: {:04X}", self.registers.get_sp()));
			ui.label(format!("PC: {:04X}", self.registers.pc));
		});

		egui::Window::new("PPU State").open(&mut self.ppu_register_window_open).show(ctx, |ui| {
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
		});
	}
}
