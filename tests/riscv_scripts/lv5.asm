  .text
  .globl main
main:
  addi sp, sp, -32
  li t0, 1
  sw t0, 0(sp)
  li t0, 2
  sw t0, 0(sp)
  li t0, 3
  sw t0, 4(sp)
  lw t0, 0(sp)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 6
  add t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 16(sp)
  addi sp, sp, 32
  ret