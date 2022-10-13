pub struct CPU {
   pub r0: u32,              // r0 = always 0
   pub r1: u32,              // r1 = assembler temporary, reserved for assembler
   pub r2: u32,              // r2, r3 = return value from a function call
   pub r3: u32,              //
   pub r4: u32,              // r4~r7 = first four parameters for a function call
   pub r5: u32,              //
   pub r6: u32,              //
   pub r7: u32,              //
   pub r8: u32,              // r8~r15 = temporary variables, need not be preserved
   pub r9: u32,              //
   pub r10: u32,             //
   pub r11: u32,             //
   pub r12: u32,             //
   pub r13: u32,             //
   pub r14: u32,             //
   pub r15: u32,             //
   pub r16: u32,             // r16~r23 = function variables, need to be preserved
   pub r17: u32,             //
   pub r18: u32,             //
   pub r19: u32,             //
   pub r20: u32,             //
   pub r21: u32,             //
   pub r22: u32,             //
   pub r23: u32,             //
   pub r24: u32,             // r24~r25 = temporary variables
   pub r25: u32,             //
   pub r26: u32,             // r26, r27 = reserved for OS
   pub r27: u32,             //
   pub r28: u32,             // r28 = global pointer
   pub r29: u32,             // r29 = stack pointer
   pub r30: u32,             // r30 = stack frame pointer or subroutine variable
   pub r31: u32,             // r31 = return address of last subroutine call
   pub memory: [u8; 65_535], // 2 ^ 16 - 1
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

