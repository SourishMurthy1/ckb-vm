use super::super::machine::Machine;
use super::super::memory::Memory;
use super::super::Error;
use super::register::Register;
use super::utils::{funct3, funct7, opcode, rd, rs1, rs2, update_register};
use super::{Execute, Instruction as GenericInstruction, Instruction::RV32M};

#[derive(Debug)]
pub enum RtypeInstruction {
    MUL,
    MULH,
    MULHSU,
    MULHU,
    DIV,
    DIVU,
    REM,
    REMU,
}

type Rtype = super::Rtype<RtypeInstruction>;

#[derive(Debug)]
pub struct Instruction(Rtype);

impl Execute for Rtype {
    fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<Option<R>, Error> {
        let rs1_value = machine.registers()[self.rs1];
        let rs2_value = machine.registers()[self.rs2];
        match &self.inst {
            RtypeInstruction::MUL => {
                let (value, _) = rs1_value.overflowing_mul(rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::MULH => {
                let value = rs1_value.overflowing_mul_high_signed(rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::MULHSU => {
                let value = rs1_value.overflowing_mul_high_signed_unsigned(rs2_value);
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::MULHU => {
                let value = rs1_value.overflowing_mul_high_unsigned(rs2_value);
                update_register(machine, self.rd, value);
            }

            // +---------------------------------------------------------------------------------------+
            // | Condition              | Dividend  | Divisor | DIVU[W] | REMU[W] |  DIV[W]   | REM[W] |
            // +------------------------+-----------+---------+---------+---------+-----------+--------+
            // | Division by zero       |     x     |    0    | 2**L-1  |    x    |    -1     |   x    |
            // +------------------------+-----------+---------+---------+---------+-----------+--------+
            // | Overflow (signed only) | −2**(L−1) |   −1    |    -    |    -    | -2**(L-1) |   0    |
            // +---------------------------------------------------------------------------------------+
            RtypeInstruction::DIV => {
                let value = if rs2_value == R::zero() {
                    // This is documented in RISC-V spec, when divided by
                    // 0, RISC-V machine would return -1 in DIV instead of
                    // trapping.
                    R::zero().overflowing_sub(R::one()).0
                } else {
                    let (value, overflow) = rs1_value.overflowing_div_signed(rs2_value);
                    if overflow {
                        R::min_value()
                    } else {
                        value
                    }
                };
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::DIVU => {
                let value = if rs2_value == R::zero() {
                    // This is documented in RISC-V spec, when divided by
                    // 0, RISC-V machine would return 2**L - 1 for unsigned integer
                    // in DIV instead of trapping.
                    R::max_value()
                } else {
                    rs1_value.overflowing_div(rs2_value).0
                };
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::REM => {
                let value = if rs2_value == R::zero() {
                    rs1_value
                } else {
                    let (value, overflow) = rs1_value.overflowing_rem_signed(rs2_value);
                    if overflow {
                        R::zero()
                    } else {
                        value
                    }
                };
                update_register(machine, self.rd, value);
            }
            RtypeInstruction::REMU => {
                let value = if rs2_value == R::zero() {
                    rs1_value
                } else {
                    rs1_value.overflowing_rem(rs2_value).0
                };
                update_register(machine, self.rd, value);
            }
        }
        Ok(None)
    }
}

impl Instruction {
    pub fn execute<Mac: Machine<R, M>, R: Register, M: Memory>(
        &self,
        machine: &mut Mac,
    ) -> Result<(), Error> {
        let next_pc = self.0.execute(machine)?;
        let default_next_pc = machine.pc().overflowing_add(R::from_usize(4)).0;
        machine.set_pc(next_pc.unwrap_or(default_next_pc));
        Ok(())
    }
}

pub fn factory(instruction_bits: u32) -> Option<GenericInstruction> {
    if opcode(instruction_bits) != 0b_0110011 || funct7(instruction_bits) != 0b_0000001 {
        None
    } else {
        let inst_opt = match funct3(instruction_bits) {
            0b_000 => Some(RtypeInstruction::MUL),
            0b_001 => Some(RtypeInstruction::MULH),
            0b_010 => Some(RtypeInstruction::MULHSU),
            0b_011 => Some(RtypeInstruction::MULHU),
            0b_100 => Some(RtypeInstruction::DIV),
            0b_101 => Some(RtypeInstruction::DIVU),
            0b_110 => Some(RtypeInstruction::REM),
            0b_111 => Some(RtypeInstruction::REMU),
            _ => None,
        };
        inst_opt.map(|inst| {
            RV32M(Instruction(Rtype {
                rd: rd(instruction_bits),
                rs1: rs1(instruction_bits),
                rs2: rs2(instruction_bits),
                inst,
            }))
        })
    }
}
