  .data
  .globl z_1
z_1:
  .word 5
 
  .data
  .globl var_1
var_1:
  .zero 4
 
  .text
  .globl main
main:
  addi sp, sp, -32
entry_1:
  li t0, 3
  sw t0, 0(sp)
  li t0, 4
  la t1, var_1
  sw t0, 0(t1)
  la t0, var_1
  lw t0, 0(t0)
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 8(sp)
  lw t0, 0(sp)
  sw t0, 12(sp)
  lw t0, 8(sp)
  lw t1, 12(sp)
  mul t0, t0, t1
  sw t0, 16(sp)
  lw a0, 16(sp)
  addi sp, sp, 32
  ret
 