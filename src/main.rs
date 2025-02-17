mod uxn;
mod assembler;

use crate::uxn::{Uxn, Opcode, InstructionMode};

fn main() {
    let mut uxn = Uxn::new();
    let lit = (Opcode::LIT as u8) | u8::from(InstructionMode::Keep);
    uxn.boot();
    uxn.load_program(&[
        lit, 0x10, Opcode::DUP as u8,
        lit, 0x20, Opcode::ADD as u8,
        lit, 0xff, lit, 0x0f, Opcode::DEO as u8, 0x00
    ], 0x100);
    let ret = uxn.eval(0x100);

    println!("{:?}", ret);
}
