  .data
  .globl a_1
a_1:
  .word 1
  .word 0
  .word 0
  .word 0
  .word 0
  .word 0
  .word 2
  .word 42
  .word 4
 
  .text
  .globl main
main:
  addi sp, sp, -96
entry:
  addi t0, sp, 0
  li t1, 0
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 24(sp)
  lw t0, 24(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 28(sp)
  li t0, 1
  lw t1, 28(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 32(sp)
  lw t0, 32(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 36(sp)
  li t0, 2
  lw t1, 36(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 40(sp)
  lw t0, 40(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 44(sp)
  li t0, 0
  lw t1, 44(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 48(sp)
  lw t0, 48(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 52(sp)
  li t0, 0
  lw t1, 52(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 56(sp)
  lw t0, 56(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 60(sp)
  li t0, 0
  lw t1, 60(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 64(sp)
  lw t0, 64(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 68(sp)
  li t0, 0
  lw t1, 68(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 12
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 72(sp)
  lw t0, 72(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 76(sp)
  lw t0, 76(sp)
  lw t0, 0(t0)
  sw t0, 80(sp)
  lw a0, 80(sp)
  addi sp, sp, 96
  ret
 