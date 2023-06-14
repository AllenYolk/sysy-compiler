  .text
  .globl main
main:
  addi sp, sp, -32
entry_1:
  li t0, 0
  li t1, 3
  xor t0, t0, t1
  seqz t0, t0
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 8(sp)
  lw t0, 8(sp)
  sw t0, 0(sp)
  lw t0, 8(sp)
  bnez t0, and_rhs_1
  j and_end_1
and_rhs_1:
  li t0, 1
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 12(sp)
  lw t0, 12(sp)
  sw t0, 0(sp)
  j and_end_1
and_end_1:
  lw t0, 0(sp)
  sw t0, 16(sp)
  li t0, 1
  lw t1, 16(sp)
  sub t0, t0, t1
  sw t0, 20(sp)
  lw a0, 20(sp)
  addi sp, sp, 32
  ret
 