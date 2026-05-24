use super::process::{Process, Registers};

pub struct Context {
    pub registers: Registers,
    pub privilege_level: u8, // 0 = Ring 0 (kernel), 3 = Ring 3 (user)
}

impl Context {
    pub fn new(rip: u64, rsp: u64, privilege_level: u8) -> Self {
        Context {
            registers: Registers {
                rip,
                rsp,
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
                rsi: 0,
                rdi: 0,
                rbp: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
            },
            privilege_level,
        }
    }

    pub fn to_registers(&self) -> Registers {
        self.registers
    }

    pub fn from_registers(regs: Registers) -> Self {
        Context {
            registers: regs,
            privilege_level: 0,
        }
    }
}

pub unsafe fn switch_context(old_context: &mut Registers, new_context: &Registers) {
    // This would be implemented in assembly in production
    // For now, just copy the registers
    *old_context = new_context.clone();
}
