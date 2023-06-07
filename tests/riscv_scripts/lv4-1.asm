  .text
  .globl main
main:
  addi sp, sp, -16
entry:
  li t0, 2
  li t1, 0
  sub t0, t0, t1
  sw t0, 0(sp)
  lw t0, 0(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 4(sp)
  lw a0, 4(sp)
  addi sp, sp, 16
  ret
 