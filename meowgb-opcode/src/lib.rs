use proc_macro::TokenStream;
use syn::{
	braced, parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, LitBool, LitInt, Token,
};

struct OpcodeImpl {
	pub cycle: LitInt,
	pub block: syn::Block,
}

impl Parse for OpcodeImpl {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let cycle = input.parse()?;
		input.parse::<syn::Token!(=>)>()?;
		let block = input.parse()?;
		Ok(Self { cycle, block })
	}
}

struct OpcodeArgs {
	pub name: syn::Ident,
	pub opcode: LitInt,
	pub readable: Expr,
	pub extended: LitBool,
	pub length: LitInt,
	pub implementation: Punctuated<OpcodeImpl, syn::Token!(,)>,
}

impl Parse for OpcodeArgs {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let name = input.parse()?;
		input.parse::<Token![,]>()?;
		let opcode = input.parse()?;
		input.parse::<Token![,]>()?;
		let readable = input.parse()?;
		input.parse::<Token![,]>()?;
		let extended = input.parse()?;
		input.parse::<Token![,]>()?;
		let length = input.parse()?;
		input.parse::<Token![,]>()?;
		let implementation_pb;
		braced!(implementation_pb in input);
		let implementation = Punctuated::parse_separated_nonempty(&implementation_pb)?;
		Ok(Self { name, opcode, readable, extended, length, implementation })
	}
}

#[proc_macro]
pub fn opcode(item: TokenStream) -> TokenStream {
	let OpcodeArgs { name, opcode, readable, extended, length, implementation } =
		parse_macro_input!(item as OpcodeArgs);

	let name_s = name.to_string();

	let opcode = opcode.base10_parse::<u8>().expect("Failed to parse opcode as u8");
	let length = length.base10_parse::<u8>().expect("Failed to parse opcode length as u8");

	let fn_sig = quote::quote! {
		pub fn #name(state: &mut Gameboy<impl crate::gameboy::serial::SerialWriter>) -> CycleResult
	};

	let mut cycle = Vec::new();
	let mut block = Vec::new();

	for op_impl in implementation {
		cycle.push(op_impl.cycle);
		block.push(op_impl.block);
	}

	/*if !cycle.is_empty() {
		if cycle[0].base10_parse::<u8>().expect("Expected u8") == 0u8 {
			block[0].stmts.insert(0, Stmt::Semi(Expr::Macro(ExprMacro::)))
		} else {

		}
	}*/

	let regs = quote::quote! {
		log::debug!("\nSTART OF {}\n-- Registers --\nAF: {:04X}\nBC: {:04X}\nDE: {:04X}\nHL: {:04X}\nSP: {:04X}\nPC: {:04X}\nZero: {}\nSubtract: {}\nHalf-Carry: {}\nCarry: {}\n-- Interrupts --\nIME: {}\nIE VBlank: {}\nIE LCD Stat: {}\nIE Timer: {}\nIE Serial: {}\nIE Joypad: {}\nIF VBlank: {}\nIF LCD Stat: {}\nIF Timer: {}\nIF Serial: {}\nIF Joypad: {}\nEND OF {}", #name_s, state.registers.get_af(), state.registers.get_bc(), state.registers.get_de(), state.registers.get_hl(), state.registers.get_sp(), state.registers.pc, state.registers.get_zero(), state.registers.get_subtract(), state.registers.get_half_carry(), state.registers.get_carry(), state.interrupts.ime, state.interrupts.read_ie_vblank(), state.interrupts.read_ie_lcd_stat(), state.interrupts.read_ie_timer(), state.interrupts.read_ie_serial(), state.interrupts.read_ie_joypad(), state.interrupts.read_if_vblank(), state.interrupts.read_if_lcd_stat(), state.interrupts.read_if_timer(), state.interrupts.read_if_serial(), state.interrupts.read_if_joypad(), #name_s);
	};

	let match_statement = quote::quote! {
		match state.registers.cycle {
			#(#cycle => {
				#block
			}),*
			cycle => unreachable!("Entered cycle {} for opcode {}", cycle, #readable),
		}
	};

	let log = if extended.value {
		quote::quote! {
			if state.registers.cycle == 1 && state.log_instructions {
				log::debug!("(PC: {:#02X}) Prefixed OP {} ({:#02X})", state.registers.pc, #readable, #opcode);
				#regs
			}
		}
	} else {
		quote::quote! {
			if state.registers.cycle == 0 && state.log_instructions {
				log::debug!("(PC: {:#02X}) OP {} ({:#02X})", state.registers.pc, #readable, #opcode);
				#regs
			}
		}
	};

	let out = quote::quote! {
		#fn_sig {
			#log

			let res: CycleResult = #match_statement;

			if res != CycleResult::NeedsMore {
				state.registers.opcode_bytecount = Some(#length);
			}

			res
		}
	};

	out.into()
}
