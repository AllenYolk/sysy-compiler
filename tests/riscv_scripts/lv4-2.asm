  .text
  .globl main
main:
  addi sp, sp, -48
entry:
  li t0, 10
  li t1, 4
  mul t0, t0, t1
  sw t0, 4(sp)
  lw t0, 4(sp)
  sw t0, 0(sp)
  li t0, 4
  li t1, 1
  sub t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  sw t0, 8(sp)
  lw t0, 0(sp)
  sw t0, 16(sp)
  lw t0, 16(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 20(sp)
  lw t0, 8(sp)
  sw t0, 24(sp)
  lw t0, 20(sp)
  lw t1, 24(sp)
  rem t0, t0, t1
  sw t0, 28(sp)
  lw t0, 28(sp)
  sw t0, 0(sp)
  lw t0, 0(sp)
  sw t0, 32(sp)
  lw t0, 32(sp)
  li t1, 4
  rem t0, t0, t1
  sw t0, 36(sp)
  lw a0, 36(sp)
  addi sp, sp, 48
  ret