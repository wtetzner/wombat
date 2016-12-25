
use bit_vec::BitVec;
use std::ops::Index;
use asm::expression::Expression;

// trait ToVal<T> {
//     fn to(&self) -> T;
// }

// impl ToVal<i8>

trait Bitable {
    fn to_bit(self) -> bool;
}

impl Bitable for bool {
    fn to_bit(self) -> bool { self }
}

impl Bitable for u8 {
    fn to_bit(self) -> bool { self == 1 }
}

macro_rules! push_bits {
    ( $bits:expr, [ $( $x:expr ),* ] ) => {
        {
            $(
                $bits.push($x.to_bit());
            )*
        }
    };
}

#[derive(Debug,Clone,Copy)]
pub struct B3 {
    value: u8
}

impl B3 {
    pub fn new(value: u8) -> Option<B3> {
        if value < 8 {
            Some(B3 {
                value: value
            })
        } else {
            None
        }
    }

    pub fn push_bits(&self, bits: &mut BitVec) {
        let byte = self.value;
        bits.push(bit_from_byte(byte, 2));
        bits.push(bit_from_byte(byte, 1));
        bits.push(bit_from_byte(byte, 0));
    }
}

#[derive(Debug,Clone,Copy)]
pub struct A12 {
    value: u16
}

impl A12 {
    pub fn new(value: u16) -> Option<A12> {
        if value < 4096 {
            Some(A12 {
                value: value
            })
        } else {
            None
        }
    }

    pub fn at(&self, pos: u8) -> bool {
        bit_from_word(self.value, pos)
    }

    pub fn push_bits(&self, bits: &mut BitVec) {
        let word = self.value;
        bits.push(bit_from_word(word, 11));
        bits.push(bit_from_word(word, 10));
        bits.push(bit_from_word(word, 9));
        bits.push(bit_from_word(word, 8));
        bits.push(bit_from_word(word, 7));
        bits.push(bit_from_word(word, 6));
        bits.push(bit_from_word(word, 5));
        bits.push(bit_from_word(word, 4));
        bits.push(bit_from_word(word, 3));
        bits.push(bit_from_word(word, 2));
        bits.push(bit_from_word(word, 1));
        bits.push(bit_from_word(word, 0));
    }
}

#[derive(Debug,Clone,Copy)]
pub struct D9 {
    value: u16
}

impl D9 {
    pub fn new(value: u16) -> Option<D9> {
        if value < 512 {
            Some(D9 {
                value: value
            })
        } else {
            None
        }
    }

    pub fn at(&self, pos: u8) -> bool {
        bit_from_word(self.value, pos)
    }

    pub fn push_bits(&self, bits: &mut BitVec) {
        let word = self.value;
        bits.push(bit_from_word(word, 8));
        bits.push(bit_from_word(word, 7));
        bits.push(bit_from_word(word, 6));
        bits.push(bit_from_word(word, 5));
        bits.push(bit_from_word(word, 4));
        bits.push(bit_from_word(word, 3));
        bits.push(bit_from_word(word, 2));
        bits.push(bit_from_word(word, 1));
        bits.push(bit_from_word(word, 0));
    }
}

#[derive(Debug,Clone,Copy)]
pub enum IndirectionMode {
    R0,
    R1,
    R2,
    R3
}

impl IndirectionMode {
    pub fn index(&self) -> u8 {
        match self {
            &IndirectionMode::R0 => 0,
            &IndirectionMode::R1 => 1,
            &IndirectionMode::R2 => 2,
            &IndirectionMode::R3 => 3,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Instruction {
    Add_i8(i8),
    Add_d9(D9),
    Add_Ri(IndirectionMode),

    Addc_i8(i8),
    Addc_d9(D9),
    Addc_Ri(IndirectionMode),

    Sub_i8(i8),
    Sub_d9(D9),
    Sub_Ri(IndirectionMode),

    Subc_i8(i8),
    Subc_d9(D9),
    Subc_Ri(IndirectionMode),

    Inc_d9(D9),
    Inc_Ri(IndirectionMode),

    Dec_d9(D9),
    Dec_Ri(IndirectionMode),

    Mul,
    Div,

    And_i8(i8),
    And_d9(D9),
    And_Ri(IndirectionMode),

    Or_i8(i8),
    Or_d9(D9),
    Or_Ri(IndirectionMode),

    Xor_i8(i8),
    Xor_d9(D9),
    Xor_Ri(IndirectionMode),

    Rol,
    Rolc,

    Ror,
    Rorc,

    Ld_d9(D9),
    Ld_Ri(IndirectionMode),

    St_d9(D9),
    St_Ri(IndirectionMode),

    Mov_d9(i8, D9),
    Mov_Rj(i8, IndirectionMode),

    Ldc,

    Push(D9),
    Pop(D9),

    Xch_d9(D9),
    Xch_Ri(IndirectionMode),

    Jmp(A12),
    Jmpf(u16),

    Br(i8),
    Brf(u16),
    Bz(i8),
    Bnz(i8),
    Bp(D9, B3, i8),
    Bpc(D9, B3, i8),
    Bn(D9, B3, i8),
    Dbnz_d9(D9, i8),
    Dbnz_Ri(IndirectionMode, i8),
    Be_i8(i8,i8),
    Be_d9(D9,i8),
    Be_Rj(IndirectionMode, i8, i8),
    Bne_i8(i8,i8),
    Bne_d9(D9,i8),
    Bne_Rj(IndirectionMode, i8, i8),

    Call(A12),
    Callf(u16),
    Callr(u16),

    Ret,
    Reti,

    Clr1(D9,B3),
    Set1(D9,B3),
    Not1(D9,B3),

    Nop
}

impl Instruction {
    #[inline]
    pub fn encode(&self, bits: &mut BitVec) {
        match self {
            &Instruction::Add_i8(i8) => { push_byte(bits, 0b10000001); push_signed_byte(bits, i8); },
            &Instruction::Add_d9(d9) => { push_bits!(bits, [ 1, 0, 0, 0, 0, 0, 1 ]); d9.push_bits(bits);},
            &Instruction::Add_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 0, 0, 0, 0, 1,  bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Addc_i8(i8) => { push_byte(bits, 0b10010001); push_signed_byte(bits, i8); },
            &Instruction::Addc_d9(d9) => { push_bits!(bits, [ 1, 0, 0, 1, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Addc_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 0, 0, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Sub_i8(i8) => { push_byte(bits, 0b10100001); push_signed_byte(bits, i8); },
            &Instruction::Sub_d9(d9) => { push_bits!(bits, [ 1, 0, 1, 0, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Sub_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 0, 1, 0, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Subc_i8(i8) => { push_byte(bits, 0b10110001); push_signed_byte(bits, i8); },
            &Instruction::Subc_d9(d9) => { push_bits!(bits, [ 1, 0, 1, 1, 0, 0, 1]); d9.push_bits(bits); },
            &Instruction::Subc_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 0, 1, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Inc_d9(d9) => { push_bits!(bits, [ 0, 1, 1, 0, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Inc_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 0, 1, 1, 0, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Dec_d9(d9) => { push_bits!(bits, [ 0, 1, 1, 1, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Dec_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 0, 1, 1, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Mul => push_byte(bits, 0b00110000),
            &Instruction::Div => push_byte(bits, 0b01000000),

            &Instruction::And_i8(i8) => { push_byte(bits, 0b11100001); push_signed_byte(bits, i8); },
            &Instruction::And_d9(d9) => { push_bits!(bits, [ 1, 1, 1, 0, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::And_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 1, 1, 0, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Or_i8(i8) => { push_byte(bits, 0b11010001); push_signed_byte(bits, i8); },
            &Instruction::Or_d9(d9) => { push_bits!(bits, [ 1, 1, 0, 1, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Or_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 1, 0, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Xor_i8(i8) => { push_byte(bits, 0b11110001); push_signed_byte(bits, i8); },
            &Instruction::Xor_d9(d9) => { push_bits!(bits, [ 1, 1, 1, 1, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Xor_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 1, 1, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Rol => push_byte(bits, 0b11100000),
            &Instruction::Rolc => push_byte(bits, 0b11110000),

            &Instruction::Ror => push_byte(bits, 0b11000000),
            &Instruction::Rorc => push_byte(bits, 0b11010000),

            &Instruction::Ld_d9(d9) => { push_bits!(bits, [ 0, 0, 0, 0, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Ld_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 0, 0, 0, 0, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::St_d9(d9) => { push_bits!(bits, [ 0, 0, 0, 1, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::St_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 0, 0, 0, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Mov_d9(i8, d9) => {
                push_bits!(bits, [ 0, 0, 1, 0, 0, 0, 1 ]);
                d9.push_bits(bits);
                push_signed_byte(bits, i8);
            },
            &Instruction::Mov_Rj(i8, rj) => {
                let index = rj.index();
                push_bits!(bits, [ 0, 0, 1, 0, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
                push_signed_byte(bits, i8);
            },

            &Instruction::Ldc => push_byte(bits, 0b11000001),

            &Instruction::Push(d9) => { push_bits!(bits, [ 0, 1, 1, 0, 0, 0, 0 ]); d9.push_bits(bits); },
            &Instruction::Pop(d9) => { push_bits!(bits, [ 0, 1, 1, 1, 0, 0, 0 ]); d9.push_bits(bits); },

            &Instruction::Xch_d9(d9) => { push_bits!(bits, [ 1, 1, 0, 0, 0, 0, 1 ]); d9.push_bits(bits); },
            &Instruction::Xch_Ri(ri) => {
                let index = ri.index();
                push_bits!(bits, [ 1, 1, 0, 0, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
            },

            &Instruction::Jmp(a12) => {
                push_bits!(bits, [ 0, 0, 1, a12.at(11), 1,
                                   a12.at(10),
                                   a12.at(9),
                                   a12.at(8),
                                   a12.at(7),
                                   a12.at(6),
                                   a12.at(5),
                                   a12.at(4),
                                   a12.at(3),
                                   a12.at(2),
                                   a12.at(1),
                                   a12.at(0)
                ]);
            },
            &Instruction::Jmpf(a16) => {
                push_byte(bits, 0b00100001);
                push_word(bits, a16);
            },

            &Instruction::Br(i8) => { push_byte(bits, 0b00000001); push_signed_byte(bits, i8); },
            &Instruction::Brf(r16) => {
                push_byte(bits, 0b00010001);
                let high_byte: u8 = (0xFF & (r16 >> 8)) as u8;
                let low_byte: u8 = (0xFF & r16) as u8;
                push_byte(bits, low_byte);
                push_byte(bits, high_byte);
            },
            &Instruction::Bz(i8) => { push_byte(bits, 0b10000000); push_signed_byte(bits, i8); },
            &Instruction::Bnz(i8) => { push_byte(bits, 0b10010000); push_signed_byte(bits, i8); },
            &Instruction::Bp(d9, b3, i8) => {
                push_bits!(bits, [ 0, 1, 1, d9.at(8), 1 ]);
                b3.push_bits(bits);
                push_bits!(bits, [ d9.at(7), d9.at(6), d9.at(5), d9.at(4), d9.at(3), d9.at(2), d9.at(1), d9.at(0) ]);
                push_signed_byte(bits, i8);
            },
            &Instruction::Bpc(d9, b3, i8) => {
                push_bits!(bits, [ 1, 0, 1, d9.at(8), 1 ]);
                b3.push_bits(bits);
                push_bits!(bits, [ d9.at(7), d9.at(6), d9.at(5), d9.at(4), d9.at(3), d9.at(2), d9.at(1), d9.at(0) ]);
                push_signed_byte(bits, i8);
            },
            &Instruction::Bn(d9, b3, i8) => {
                push_bits!(bits, [ 1, 0, 0, d9.at(8), 1 ]);
                b3.push_bits(bits);
                push_bits!(bits, [ d9.at(7), d9.at(6), d9.at(5), d9.at(4), d9.at(3), d9.at(2), d9.at(1), d9.at(0) ]);
                push_signed_byte(bits, i8);
            },
            &Instruction::Dbnz_d9(d9, i8) => {
                push_bits!(bits, [ 0, 1, 0, 1, 0, 0, 1 ]);
                d9.push_bits(bits);
                push_signed_byte(bits, i8);
            },
            &Instruction::Dbnz_Ri(ri, i8) => {
                let index = ri.index();
                push_bits!(bits, [ 0, 1, 0, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
                push_signed_byte(bits, i8);
            },
            &Instruction::Be_i8(i8,r8) => {
                push_byte(bits, 0b00110001);
                push_signed_byte(bits, i8);
                push_signed_byte(bits, r8);
            },
            &Instruction::Be_d9(d9,i8) => {
                push_bits!(bits, [ 0, 0, 1, 1, 0, 0, 1 ]);
                d9.push_bits(bits);
                push_signed_byte(bits, i8);
            },
            &Instruction::Be_Rj(rj, i8, r8) => {
                let index = rj.index();
                push_bits!(bits, [ 0, 0, 1, 1, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
                push_signed_byte(bits, i8);
                push_signed_byte(bits, r8);
            },
            &Instruction::Bne_i8(i8,r8) => {
                push_byte(bits, 0b01000001);
                push_signed_byte(bits, i8);
                push_signed_byte(bits, r8);
            },
            &Instruction::Bne_d9(d9,r8) => {
                push_bits!(bits, [ 0, 1, 0, 0, 0, 0, 1 ]);
                d9.push_bits(bits);
                push_signed_byte(bits, r8);
            },
            &Instruction::Bne_Rj(rj, i8, r8) => {
                let index = rj.index();
                push_bits!(bits, [ 0, 1, 0, 0, 0, 1, bit_from_byte(index, 1), bit_from_byte(index, 0) ]);
                push_signed_byte(bits, i8);
                push_signed_byte(bits, r8);
            },

            &Instruction::Call(a12) => {
                push_bits!(bits, [
                    0, 0, 0, a12.at(11), 1,
                    a12.at(10),
                    a12.at(9),
                    a12.at(8),
                    a12.at(7),
                    a12.at(6),
                    a12.at(5),
                    a12.at(4),
                    a12.at(3),
                    a12.at(2),
                    a12.at(1),
                    a12.at(0)
                ]);
            },
            &Instruction::Callf(a16) => {
                push_byte(bits, 0b00100000);
                push_word(bits, a16);
            },
            &Instruction::Callr(r16) => {
                push_byte(bits, 0b00010000);
                push_word(bits, r16);
            },

            &Instruction::Ret => push_byte(bits, 0b10100000),
            &Instruction::Reti => push_byte(bits, 0b10110000),

            &Instruction::Clr1(d9,b3) => {
                push_bits!(bits, [ 1, 1, 0, d9.at(8), 1 ]);
                b3.push_bits(bits);
                push_bits!(bits, [ d9.at(7), d9.at(6), d9.at(5), d9.at(4), d9.at(3), d9.at(2), d9.at(1), d9.at(0) ]);
            },
            &Instruction::Set1(d9,b3) => {
                push_bits!(bits, [ 1, 1, 1, d9.at(8), 1 ]);
                b3.push_bits(bits);
                push_bits!(bits, [ d9.at(7), d9.at(6), d9.at(5), d9.at(4), d9.at(3), d9.at(2), d9.at(1), d9.at(0) ]);
            },
            &Instruction::Not1(d9,b3) => {
                push_bits!(bits, [ 1, 0, 1, d9.at(8), 1 ]);
                b3.push_bits(bits);
                push_bits!(bits, [ d9.at(7), d9.at(6), d9.at(5), d9.at(4), d9.at(3), d9.at(2), d9.at(1), d9.at(0) ]);
            },

            &Instruction::Nop => push_byte(bits, 0)
        }
    }

    /// The size in bytes of the instruction
    #[inline]
    pub fn size(&self) -> usize {
        match self {
            &Instruction::Add_i8(_) => 2,
            &Instruction::Add_d9(_) => 2,
            &Instruction::Add_Ri(_) => 1,

            &Instruction::Addc_i8(_) => 2,
            &Instruction::Addc_d9(_) => 2,
            &Instruction::Addc_Ri(_) => 1,

            &Instruction::Sub_i8(_) => 2,
            &Instruction::Sub_d9(_) => 2,
            &Instruction::Sub_Ri(_) => 1,

            &Instruction::Subc_i8(_) => 2,
            &Instruction::Subc_d9(_) => 2,
            &Instruction::Subc_Ri(_) => 1,

            &Instruction::Inc_d9(_) => 2,
            &Instruction::Inc_Ri(_) => 1,

            &Instruction::Dec_d9(_) => 2,
            &Instruction::Dec_Ri(_) => 1,

            &Instruction::Mul => 1,
            &Instruction::Div => 1,

            &Instruction::And_i8(_) => 2,
            &Instruction::And_d9(_) => 2,
            &Instruction::And_Ri(_) => 1,

            &Instruction::Or_i8(_) => 2,
            &Instruction::Or_d9(_) => 2,
            &Instruction::Or_Ri(_) => 1,

            &Instruction::Xor_i8(_) => 2,
            &Instruction::Xor_d9(_) => 2,
            &Instruction::Xor_Ri(_) => 1,

            &Instruction::Rol => 1,
            &Instruction::Rolc => 1,

            &Instruction::Ror => 1,
            &Instruction::Rorc => 1,

            &Instruction::Ld_d9(_) => 2,
            &Instruction::Ld_Ri(_) => 1,

            &Instruction::St_d9(_) => 2,
            &Instruction::St_Ri(_) => 1,

            &Instruction::Mov_d9(_, _) => 3,
            &Instruction::Mov_Rj(_, _) => 2,

            &Instruction::Ldc => 1,

            &Instruction::Push(_) => 2,
            &Instruction::Pop(_) => 2,

            &Instruction::Xch_d9(_) => 2,
            &Instruction::Xch_Ri(_) => 1,

            &Instruction::Jmp(_) => 2,
            &Instruction::Jmpf(_) => 3,

            &Instruction::Br(_) => 2,
            &Instruction::Brf(_) => 3,
            &Instruction::Bz(_) => 2,
            &Instruction::Bnz(_) => 2,
            &Instruction::Bp(_, _, _) => 3,
            &Instruction::Bpc(_, _, _) => 3,
            &Instruction::Bn(_, _, _) => 3,
            &Instruction::Dbnz_d9(_, _) => 3,
            &Instruction::Dbnz_Ri(_, _) => 2,
            &Instruction::Be_i8(_,_) => 3,
            &Instruction::Be_d9(_,_) => 3,
            &Instruction::Be_Rj(_, _, _) => 3,
            &Instruction::Bne_i8(_,_) => 3,
            &Instruction::Bne_d9(_,_) => 3,
            &Instruction::Bne_Rj(_, _, _) => 3,

            &Instruction::Call(_) => 2,
            &Instruction::Callf(_) => 3,
            &Instruction::Callr(_) => 3,

            &Instruction::Ret => 1,
            &Instruction::Reti => 1,

            &Instruction::Clr1(_,_) => 2,
            &Instruction::Set1(_,_) => 2,
            &Instruction::Not1(_,_) => 2,

            &Instruction::Nop => 1
        }
    }
}

#[inline]
fn push_signed_byte(bits: &mut BitVec, byte: i8) {
    bits.push(bit_from_signed_byte(byte, 7));
    bits.push(bit_from_signed_byte(byte, 6));
    bits.push(bit_from_signed_byte(byte, 5));
    bits.push(bit_from_signed_byte(byte, 4));
    bits.push(bit_from_signed_byte(byte, 3));
    bits.push(bit_from_signed_byte(byte, 2));
    bits.push(bit_from_signed_byte(byte, 1));
    bits.push(bit_from_signed_byte(byte, 0));
}

#[inline]
fn push_byte(bits: &mut BitVec, byte: u8) {
    bits.push(bit_from_byte(byte, 7));
    bits.push(bit_from_byte(byte, 6));
    bits.push(bit_from_byte(byte, 5));
    bits.push(bit_from_byte(byte, 4));
    bits.push(bit_from_byte(byte, 3));
    bits.push(bit_from_byte(byte, 2));
    bits.push(bit_from_byte(byte, 1));
    bits.push(bit_from_byte(byte, 0));
}

#[inline]
fn bit_from_byte(input: u8, pos: u8) -> bool {
    if pos < 8 {
        input & (1 << pos) != 0
    } else {
        false
    }
}

#[inline]
fn bit_from_signed_byte(input: i8, pos: u8) -> bool {
    if pos < 8 {
        input & (1 << pos) != 0
    } else {
        false
    }
}

#[inline]
fn push_word(bits: &mut BitVec, word: u16) {
    bits.push(bit_from_word(word, 15));
    bits.push(bit_from_word(word, 14));
    bits.push(bit_from_word(word, 13));
    bits.push(bit_from_word(word, 12));
    bits.push(bit_from_word(word, 11));
    bits.push(bit_from_word(word, 10));
    bits.push(bit_from_word(word, 9));
    bits.push(bit_from_word(word, 8));
    bits.push(bit_from_word(word, 7));
    bits.push(bit_from_word(word, 6));
    bits.push(bit_from_word(word, 5));
    bits.push(bit_from_word(word, 4));
    bits.push(bit_from_word(word, 3));
    bits.push(bit_from_word(word, 2));
    bits.push(bit_from_word(word, 1));
    bits.push(bit_from_word(word, 0));
}

#[inline]
fn bit_from_word(input: u16, pos: u8) -> bool {
    if pos < 16 {
        input & (1 << pos) != 0
    } else {
        false
    }
}

#[cfg(test)]
mod Test {
    use bit_vec::BitVec;
    use asm::instruction::Instruction::*;
    use asm::instruction::IndirectionMode::*;
    use asm::instruction::*;

    #[test]
    fn test_count_with_encoding() {

        let instructions = vec![
            Add_i8(15),
            Add_d9(D9::new(32).expect("Invalid d9 value")),
            Add_Ri(R0),

            Addc_i8(-126),
            Addc_d9(D9::new(12).expect("Invalid d9 value")),
            Addc_Ri(R1),

            Sub_i8(-1),
            Sub_d9(D9::new(80).expect("Invalid d9 value")),
            Sub_Ri(R2),

            Subc_i8(3),
            Subc_d9(D9::new(511).expect("Invalid d9 value")),
            Subc_Ri(R3),

            Inc_d9(D9::new(12).expect("Invalid d9 value")),
            Inc_Ri(R0),

            Dec_d9(D9::new(5).expect("Invalid d9 value")),
            Dec_Ri(R1),

            Mul,
            Div,

            And_i8(4),
            And_d9(D9::new(7).expect("Invalid d9 value")),
            And_Ri(R2),

            Or_i8(0),
            Or_d9(D9::new(17).expect("Invalid d9 value")),
            Or_Ri(R3),

            Xor_i8(1),
            Xor_d9(D9::new(12).expect("Invalid d9 value")),
            Xor_Ri(R0),

            Rol,
            Rolc,

            Ror,
            Rorc,

            Ld_d9(D9::new(11).expect("Invalid d9 value")),
            Ld_Ri(R1),

            St_d9(D9::new(17).expect("Invalid d9 value")),
            St_Ri(R2),

            Mov_d9(9, D9::new(23).expect("Invalid d9 value")),
            Mov_Rj(14, R3),

            Ldc,

            Push(D9::new(45).expect("Invalid d9 value")),
            Pop(D9::new(54).expect("Invalid d9 value")),

            Xch_d9(D9::new(18).expect("Invalid d9 value")),
            Xch_Ri(R0),

            Jmp(A12::new(4000).expect("Invalid a12 value")),
            Jmpf(700),

            Br(75),
            Brf(123),
            Bz(22),
            Bnz(0),
            Bp(D9::new(32).expect("Invalid d9 value"), B3::new(7).expect("Invalid b3 value"), 12),
            Bpc(D9::new(32).expect("Invalid d9 value"), B3::new(7).expect("Invalid b3 value"), 12),
            Bn(D9::new(32).expect("Invalid d9 value"), B3::new(7).expect("Invalid b3 value"), 12),
            Dbnz_d9(D9::new(76).expect("Invalid d9 value"), 32),
            Dbnz_Ri(R1, 98),
            Be_i8(89,89),
            Be_d9(D9::new(4).expect("Invalid d9 value"),17),
            Be_Rj(R2, 4, 5),
            Bne_i8(6,7),
            Bne_d9(D9::new(8).expect("Invalid d9 value"),9),
            Bne_Rj(R3, 10, 11),

            Call(A12::new(3000).expect("Invalid a12 value")),
            Callf(100),
            Callr(2000),

            Ret,
            Reti,

            Clr1(D9::new(32).expect("Invalid d9 value"),B3::new(6).expect("Invalid b3 value")),
            Set1(D9::new(32).expect("Invalid d9 value"),B3::new(6).expect("Invalid b3 value")),
            Not1(D9::new(32).expect("Invalid d9 value"),B3::new(6).expect("Invalid b3 value")),

            Nop
        ];

        for inst in instructions.iter() {
            let mut bits = BitVec::with_capacity(500);
            inst.encode(&mut bits);
            let bytes = bits.to_bytes();
            println!("inst: {:?}", inst);
            assert!(bytes.len() == inst.size());
        }

        let mut expected_len = 0;
        for inst in instructions.iter() {
            expected_len = expected_len + inst.size();
        }

        let mut bits = BitVec::with_capacity(500);
        for inst in instructions.iter() {
            inst.encode(&mut bits);
        }

        let results = bits.to_bytes();

        println!("Expected len: {}, encoded len: {}", expected_len, results.len());
        assert!(expected_len == results.len());
    }
}
