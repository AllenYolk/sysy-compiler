  .text
  .globl main
main:
  addi sp, sp, -96
entry:
  li t0, 2
  sw t0, 0(sp)
  lw t0, 0(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  beqz t0, if_then_1
  j if_else_1
if_then_1:
  lw t0, 0(sp)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  sw t0, 0(sp)
  lw t0, 0(sp)
  sw t0, 20(sp)
  li t0, 2
  lw t1, 20(sp)
  mul t0, t0, t1
  sw t0, 24(sp)
  lw t0, 24(sp)
  sw t0, 16(sp)
  li t0, 0
  sw t0, 28(sp)
  lw t0, 28(sp)
  sw t0, 32(sp)
  lw t0, 32(sp)
  beqz t0, if_then_2
  j if_else_2
if_else_1:
  li t0, 4
  sw t0, 0(sp)
  j if_end_1
if_then_2:
  lw t0, 16(sp)
  sw t0, 36(sp)
  lw t0, 36(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  sw t0, 16(sp)
  lw t0, 16(sp)
  sw t0, 44(sp)
  lw t0, 44(sp)
  li t1, 0
  slt t0, t0, t1
  sw t0, 48(sp)
  lw t0, 48(sp)
  beqz t0, if_then_3
  j if_else_3
if_else_2:
  lw t0, 16(sp)
  sw t0, 52(sp)
  lw t0, 52(sp)
  li t1, 4
  rem t0, t0, t1
  sw t0, 56(sp)
  lw t0, 56(sp)
  sw t0, 16(sp)
  j if_end_2
if_end_1:
  lw t0, 0(sp)
  sw t0, 60(sp)
  lw a0, 60(sp)
  addi sp, sp, 96
  ret
if_then_3:
  lw t0, 16(sp)
  sw t0, 64(sp)
  li t0, 0
  lw t1, 64(sp)
  sub t0, t0, t1
  sw t0, 68(sp)
  lw t0, 68(sp)
  sw t0, 16(sp)
  j if_end_3
if_else_3:
  j if_end_3
if_end_2:
  lw t0, 28(sp)
  sw t0, 72(sp)
  lw t0, 16(sp)
  sw t0, 76(sp)
  lw t0, 72(sp)
  lw t1, 76(sp)
  add t0, t0, t1
  sw t0, 80(sp)
  lw t0, 80(sp)
  sw t0, 28(sp)
  j if_end_1
if_end_3:
  j if_end_2