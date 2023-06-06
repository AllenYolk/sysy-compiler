  .text
  .globl main
main:
  addi sp, sp, -64
entry:
  li t0, 0
  sw t0, 0(sp)
  li t0, 1
  sw t0, 4(sp)
  lw t0, 4(sp)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 10
  slt t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  beqz t0, if_then_1
  j if_else_1
if_then_1:
  j while_entry_1
if_else_1:
  j if_end_1
while_entry_1:
  lw t0, 0(sp)
  sw t0, 16(sp)
  lw t0, 16(sp)
  li t1, 3
  slt t0, t0, t1
  sw t0, 20(sp)
  lw t0, 20(sp)
  beqz t0, while_body_1
  j while_end_1
if_end_1:
  lw t0, 4(sp)
  sw t0, 24(sp)
  lw a0, 24(sp)
  addi sp, sp, 64
  ret
while_body_1:
  lw t0, 4(sp)
  sw t0, 28(sp)
  lw t0, 28(sp)
  li t1, 10
  slt t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  beqz t0, if_then_2
  j if_else_2
while_end_1:
  j if_end_1
if_then_2:
  lw t0, 4(sp)
  sw t0, 36(sp)
  lw t0, 4(sp)
  sw t0, 40(sp)
  lw t0, 36(sp)
  lw t1, 40(sp)
  mul t0, t0, t1
  sw t0, 44(sp)
  lw t0, 44(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 48(sp)
  lw t0, 48(sp)
  sw t0, 4(sp)
  j if_end_2
if_else_2:
  lw t0, 0(sp)
  sw t0, 52(sp)
  lw t0, 52(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 56(sp)
  lw t0, 56(sp)
  sw t0, 0(sp)
  j if_end_2
if_end_2:
  j while_entry_1