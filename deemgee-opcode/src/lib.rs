use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{
	braced, parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, ExprMacro, LitBool,
	LitInt, LitStr, Stmt, Token,
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
		let implementation_pb;
		braced!(implementation_pb in input);
		let implementation = Punctuated::parse_separated_nonempty(&implementation_pb)?;
		Ok(Self { name, opcode, readable, extended, implementation })
	}
}

#[proc_macro]
pub fn opcode(item: TokenStream) -> TokenStream {
	let OpcodeArgs { name, opcode, readable, extended, implementation } =
		parse_macro_input!(item as OpcodeArgs);

	let opcode = opcode.base10_parse::<u8>().expect("Failed to parse opcode as u8");

	let fn_sig = quote::quote! {
		pub fn #name(state: &mut Gameboy) -> CycleResult
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
		log::trace!("-- Registers --\nAF: {:04X}\nBC: {:04X}\nDE: {:04X}\nHL: {:04X}\nSP: {:04X}\nPC: {:04X}\nZero: {}\nSubtract: {}\nHalf-Carry: {}\nCarry: {}", state.registers.get_af(), state.registers.get_bc(), state.registers.get_de(), state.registers.get_hl(), state.registers.get_sp(), state.registers.pc, state.registers.get_zero(), state.registers.get_subtract(), state.registers.get_half_carry(), state.registers.get_carry());
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
				log::debug!("Prefixed OP {} ({:#02X})", #readable, #opcode);
			}
		}
	} else {
		quote::quote! {
			if state.registers.cycle == 0 && state.log_instructions {
				log::debug!("OP {} ({:#02X})", #readable, #opcode);
			}
		}
	};

	let out = quote::quote! {
		#fn_sig {
			#log
			#regs

			#match_statement
		}
	};

	out.into()
}
