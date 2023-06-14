  .text
  .globl half
half:
  addi sp, sp, -16
entry_1:
  sw a0, 0(sp)
  lw t0, 0(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 2
  div t0, t0, t1
  sw t0, 8(sp)
  lw a0, 8(sp)
  addi sp, sp, 16
  ret
 
  .text
  .globl f
f:
  # no prologue
entry_2:
  # no epilogue
  ret
 
  .text
  .globl main
main:
  addi sp, sp, -16
  sw ra, 12(sp)
entry_3:
  call f
  li a0, 10
  call half
  sw a0, 4(sp)
  lw a0, 4(sp)
  lw ra, 12(sp)
  addi sp, sp, 16
  ret
 