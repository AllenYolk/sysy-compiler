  .text
  .globl main
main:
  addi sp, sp, -112
entry_1:
  li t0, 0
  sw t0, 0(sp)
  li t0, 1
  sw t0, 4(sp)
  lw t0, 0(sp)
  sw t0, 12(sp)
  lw t0, 12(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 8(sp)
  lw t0, 16(sp)
  bnez t0, or_end_1
  j or_rhs_1
or_end_1:
  lw t0, 8(sp)
  sw t0, 20(sp)
  lw t0, 20(sp)
  bnez t0, if_then_1
  j if_else_1
or_rhs_1:
  lw t0, 4(sp)
  sw t0, 24(sp)
  lw t0, 24(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 28(sp)
  lw t0, 28(sp)
  sw t0, 8(sp)
  j or_end_1
if_then_1:
  lw t0, 0(sp)
  sw t0, 32(sp)
  lw t0, 4(sp)
  sw t0, 36(sp)
  lw t0, 32(sp)
  lw t1, 36(sp)
  add t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  sw t0, 0(sp)
  j if_end_1
if_else_1:
  lw t0, 4(sp)
  sw t0, 48(sp)
  li t0, 0
  lw t1, 48(sp)
  xor t0, t0, t1
  seqz t0, t0
  sw t0, 52(sp)
  lw t0, 52(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 56(sp)
  lw t0, 56(sp)
  sw t0, 44(sp)
  lw t0, 56(sp)
  bnez t0, or_end_2
  j or_rhs_2
if_end_1:
  lw t0, 0(sp)
  sw t0, 60(sp)
  lw a0, 60(sp)
  addi sp, sp, 112
  ret
or_end_2:
  lw t0, 44(sp)
  sw t0, 64(sp)
  lw t0, 64(sp)
  sw t0, 0(sp)
  j if_end_1
or_rhs_2:
  lw t0, 0(sp)
  sw t0, 72(sp)
  li t0, 0
  lw t1, 72(sp)
  xor t0, t0, t1
  seqz t0, t0
  sw t0, 76(sp)
  lw t0, 76(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 80(sp)
  lw t0, 80(sp)
  sw t0, 68(sp)
  lw t0, 80(sp)
  bnez t0, and_rhs_1
  j and_end_1
and_rhs_1:
  lw t0, 4(sp)
  sw t0, 84(sp)
  lw t0, 84(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 88(sp)
  lw t0, 88(sp)
  sw t0, 68(sp)
  j and_end_1
and_end_1:
  lw t0, 68(sp)
  sw t0, 92(sp)
  lw t0, 92(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 96(sp)
  lw t0, 96(sp)
  sw t0, 44(sp)
  j or_end_2
 