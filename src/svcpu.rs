pub struct CPU {
    pub registers: [u32; 32],
    // r0 = always 0
    // r1 = assembler temporary, reserved for assembler
    // r2, r3 = return value from a function call
    // r4~r7 = first four parameters for a function call
    // r8~r15 = temporary variables, need not be preserved
    // r16~r23 = function variables, need to be preserved
    // r24~r25 = temporary variables
    // r26, r27 = reserved for OS
    // r28 = global pointer
    // r29 = stack pointer
    // r30 = stack frame pointer or subroutine variable
    // r31 = return address of last subroutine call
    pub memory: [u8; 65_535], // 2 ^ 16 - 1
}
impl Default for CPU {
    fn default() -> CPU {
        return CPU {
            registers: [0; 32],
            memory: [0; 65_535],
        };
    }
}
