use core::panic;

use byteorder::{BigEndian, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};

pub struct Instructions {
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, FromPrimitive, ToPrimitive, Copy)]
pub enum Opcode {
    Constant,
}

impl Opcode {
    pub fn width_lookup(&self) -> Vec<u32> {
        match self {
            Opcode::Constant => vec![2],
        }
    }

    pub fn make(&self, operands: Vec<i32>) -> Instructions {
        let widths = self.width_lookup();
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(*self as u8);

        for (op, width) in operands.iter().zip(widths) {
            match width {
                2 => instructions.write_u16::<BigEndian>(*op as u16).unwrap(),
                1 => instructions.write_u8(*op as u8).unwrap(),
                _ => panic!("What width is this => {width}"),
            }
        }

        Instructions { data: instructions }
    }
}
