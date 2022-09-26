struct CPU {
    r0: i32,              // r0 = always 0
    r1: i32,              // r1 = assembler temporary, reserved for assembler
    r2: i32,              // r2, r3 = return value from a function call
    r3: i32,              //
    r4: i32,              // r4~r7 = first four parameters for a function call
    r5: i32,              //
    r6: i32,              //
    r7: i32,              //
    r8: i32,              // r8~r15 = temporary variables, need not be preserved
    r9: i32,              //
    r10: i32,             //
    r11: i32,             //
    r12: i32,             //
    r13: i32,             //
    r14: i32,             //
    r15: i32,             //
    r16: i32,             // r16~r23 = function variables, need to be preserved
    r17: i32,             //
    r18: i32,             //
    r19: i32,             //
    r20: i32,             //
    r21: i32,             //
    r22: i32,             //
    r23: i32,             //
    r24: i32,             // r24~r25 = temporary variables
    r25: i32,             //
    r26: i32,             // r26, r27 = reserved for OS
    r27: i32,             //
    r28: i32,             // r28 = global pointer
    r29: i32,             // r29 = stack pointer
    r30: i32,             // r30 = stack frame pointer or subroutine variable
    r31: i32,             // r31 = return address of last subroutine call
    memory: [u8; 65_535], // 2 ^ 16 - 1
}
impl Default for CPU {
    fn default() -> CPU {
        return CPU {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            r16: 0,
            r17: 0,
            r18: 0,
            r19: 0,
            r20: 0,
            r21: 0,
            r22: 0,
            r23: 0,
            r24: 0,
            r25: 0,
            r26: 0,
            r27: 0,
            r28: 0,
            r29: 0,
            r30: 0,
            r31: 0,
            memory: [0; 65_535]
        };
    }
}

fn main() {
    let cpu = CPU { ..Default::default() };
    println!("Hello, world!");
}
