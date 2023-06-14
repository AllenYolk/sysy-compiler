  .data
  .globl arr_1
arr_1:
  .zero 24
 
  .text
  .globl main
main:
  addi sp, sp, -48
entry_1:
  addi t0, sp, 0
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 16(sp)
  li t0, 1
  lw t1, 16(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 20(sp)
  li t0, 0
  lw t1, 20(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 24(sp)
  li t0, 0
  lw t1, 24(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 28(sp)
  li t0, 0
  lw t1, 28(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  lw t0, 0(t0)
  sw t0, 36(sp)
  lw a0, 36(sp)
  addi sp, sp, 48
  ret
 