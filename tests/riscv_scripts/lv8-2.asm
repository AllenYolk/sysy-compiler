  .text
  .globl main
main:
  addi sp, sp, -16
  sw ra, 12(sp)
entry_1:
  call getint
  sw a0, 0(sp)
  lw a0, 0(sp)
  lw ra, 12(sp)
  addi sp, sp, 16
  ret
 