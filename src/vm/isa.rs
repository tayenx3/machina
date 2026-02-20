/// No operation
/// `[8:opcode]`
pub const NOP: u8  = 0x00;
/// Halt machine
/// `[8:opcode]`
pub const HLT: u8  = 0x01;
/// Load immediate
/// `[8:opcode][4:dest][32:imm]`
pub const LDI: u8  = 0x02;
/// Add
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const ADD: u8  = 0x03;
/// Subtract
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const SUB: u8  = 0x04;
/// Bitwise OR
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const BOR: u8  = 0x05;
/// Bitwise AND
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const BAND: u8 = 0x06;
/// Bitwise XOR
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const BXOR: u8 = 0x07;
/// Bitwise NOT
/// `[8:opcode][4:dest][4:src]`
pub const BNOT: u8 = 0x08;
/// Logical OR
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const LOR: u8  = 0x09;
/// Logical AND
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const LAND: u8 = 0x0A;
/// Logical XOR
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const LXOR: u8 = 0x0B;
/// Logical NOT
/// `[8:opcode][4:dest][4:src]`
pub const LNOT: u8 = 0x0C;
/// Store byte
/// `[8:opcode][4:dest][4:src]`
pub const SB: u8   = 0x0D;
/// Store word
/// `[8:opcode][4:dest][4:src]`
pub const SW: u8   = 0x0E;
/// Load byte signed
/// `[8:opcode][4:dest][4:src]`
pub const LBS: u8  = 0x0F;
/// Load byte unsigned
/// `[8:opcode][4:dest][4:src]`
pub const LBU: u8  = 0x10;
/// Load word
/// `[8:opcode][4:dest][4:src]`
pub const LW: u8   = 0x11;
/// Jump
/// `[8:opcode][4:dest][1:rel/abs]`
pub const JMP: u8  = 0x12;
/// Branch if true
/// `[8:opcode][4:dest][4:src][1:rel/abs]`
pub const BRIF: u8 = 0x13;
/// Call
/// `[8:opcode][4:dest][1:rel/abs]`
pub const CAL: u8  = 0x14;
/// Call if true
/// `[8:opcode][4:dest][4:src][1:rel/abs]`
pub const CAIF: u8 = 0x15;
/// Return
/// `[8:opcode]`
pub const RET: u8  = 0x16;
/// Compare equal
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const EQ: u8   = 0x17;
/// Compare not-equal
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const NE: u8   = 0x18;
/// Compare greater-than
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const GT: u8   = 0x19;
/// Compare less-than
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const LT: u8   = 0x1A;
/// Compare greater-than-or-equal
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const GE: u8   = 0x1B;
/// Compare less-than-or-equal
/// `[8:opcode][4:dest][4:src1][4:src2]`
pub const LE: u8   = 0x1C;