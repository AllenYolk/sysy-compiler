  .text
  .globl main
main:
  addi sp, sp, -16
entry:
  li t0, 0
  li t1, 6
  xor t0, t0, t1
  seqz t0, t0
  sw t0, 0(sp)
  li t0, 0
  lw t1, 0(sp)
  sub t0, t0, t1
  sw t0, 4(sp)
  li t0, 0
  lw t1, 4(sp)
  sub t0, t0, t1
  sw t0, 8(sp)
  lw a0, 8(sp)
  addi sp, sp, 16
  ret
 