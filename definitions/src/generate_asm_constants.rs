use ckb_vm_definitions::{
    asm::{
        AsmCoreMachine, Trace, RET_DECODE_TRACE, RET_EBREAK, RET_ECALL, RET_MAX_CYCLES_EXCEEDED,
        TRACE_ITEM_LENGTH,
    },
    instructions::{Instruction, INSTRUCTION_OPCODE_NAMES},
    registers::SP,
};
use std::mem::{size_of, zeroed};

// This utility helps us generate C-based macros containing definitions
// such as return code, opcode, struct size, struct offset, etc. The exact
// data here are derived while inspecting Rust structs dynamically. We keep
// this in a separate crate so build failures from the main crate won't cause
// a problem when updating the definitions with this crate. Or you can think
// of this as a workaround to the problem that build.rs cannot depend on any
// of its crate contents.
fn main() {
    println!(
        "#define CKB_VM_ASM_MAXIMUM_TRACE_ADDRESS_LENGTH {}",
        TRACE_ITEM_LENGTH * 4
    );
    println!();

    println!("#define CKB_VM_ASM_RET_DECODE_TRACE {}", RET_DECODE_TRACE);
    println!("#define CKB_VM_ASM_RET_ECALL {}", RET_ECALL);
    println!("#define CKB_VM_ASM_RET_EBREAK {}", RET_EBREAK);
    println!(
        "#define CKB_VM_ASM_RET_MAX_CYCLES_EXCEEDED {}",
        RET_MAX_CYCLES_EXCEEDED
    );
    println!();

    println!("#define CKB_VM_ASM_REGISTER_SP {}", SP);
    println!();

    println!(
        "#define CKB_VM_ASM_TRACE_STRUCT_SIZE {}",
        size_of::<Trace>()
    );

    let t: Trace = unsafe { zeroed() };
    let t_address = &t as *const Trace as usize;
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_ADDRESS {}",
        (&t.address as *const u64 as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_LENGTH {}",
        (&t.length as *const u8 as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_CYCLES {}",
        (&t.cycles as *const u64 as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_INSTRUCTIONS {}",
        (&t.instructions as *const Instruction as usize) - t_address
    );
    println!(
        "#define CKB_VM_ASM_TRACE_OFFSET_THREAD {}",
        (&t.thread as *const u64 as usize) - t_address
    );
    println!();

    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_STRUCT_SIZE {}",
        size_of::<AsmCoreMachine>()
    );

    let m: Box<AsmCoreMachine> = Box::<AsmCoreMachine>::default();
    let m_address = &*m as *const AsmCoreMachine as usize;
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_REGISTERS {}",
        (&m.registers as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_PC {}",
        (&m.pc as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_CYCLES {}",
        (&m.cycles as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MAX_CYCLES {}",
        (&m.max_cycles as *const u64 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_ELF_END {}",
        (&m.elf_end as *const usize as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_MEMORY {}",
        (&m.memory as *const u8 as usize) - m_address
    );
    println!(
        "#define CKB_VM_ASM_ASM_CORE_MACHINE_OFFSET_TRACES {}",
        (&m.traces as *const Trace as usize) - m_address
    );
    println!();

    for (op, name) in INSTRUCTION_OPCODE_NAMES.iter().enumerate() {
        println!("#define CKB_VM_ASM_OP_{} {}", name, op);
    }
    println!();

    println!("#ifdef CKB_VM_ASM_GENERATE_LABEL_TABLES");
    println!("#ifdef __APPLE__");
    println!(".global _ckb_vm_asm_labels");
    println!("_ckb_vm_asm_labels:");
    println!("#else");
    println!(".global ckb_vm_asm_labels");
    println!("ckb_vm_asm_labels:");
    println!("#endif");
    println!(".CKB_VM_ASM_LABEL_TABLE:");
    for name in INSTRUCTION_OPCODE_NAMES.iter() {
        println!(
            "\t.long\t.CKB_VM_ASM_LABEL_OP_{} - .CKB_VM_ASM_LABEL_TABLE",
            name
        );
    }
    println!("#endif /* CKB_VM_ASM_GENERATE_LABEL_TABLES */");
}
