  .data
  .globl x_1
x_1:
  .word 10
  .word 20
 
  .data
  .globl xx_1
xx_1:
  .word 9
 
  .data
  .globl y_1
y_1:
  .word 114
  .word 0
  .word 0
 
  .data
  .globl q_1
q_1:
  .zero 20
 
  .text
  .globl main
main:
  addi sp, sp, -128
entry:
  la t0, xx_1
  lw t0, 0(t0)
  sw t0, 0(sp)
  lw t0, 0(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 4(sp)
  addi t0, sp, 8
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 28(sp)
  li t0, 1
  lw t1, 28(sp)
  sw t0, 0(t1)
  addi t0, sp, 8
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 32(sp)
  li t0, 2
  lw t1, 32(sp)
  sw t0, 0(t1)
  addi t0, sp, 8
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 36(sp)
  lw t0, 4(sp)
  lw t1, 36(sp)
  sw t0, 0(t1)
  addi t0, sp, 8
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 40(sp)
  li t0, 0
  lw t1, 40(sp)
  sw t0, 0(t1)
  addi t0, sp, 8
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 44(sp)
  li t0, 0
  lw t1, 44(sp)
  sw t0, 0(t1)
  addi t0, sp, 48
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 60(sp)
  li t0, 5
  lw t1, 60(sp)
  sw t0, 0(t1)
  addi t0, sp, 48
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 64(sp)
  li t0, 0
  lw t1, 64(sp)
  sw t0, 0(t1)
  addi t0, sp, 48
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 68(sp)
  li t0, 0
  lw t1, 68(sp)
  sw t0, 0(t1)
  li t0, 1
  li t1, 1
  add t0, t0, t1
  sw t0, 84(sp)
  addi t0, sp, 8
  lw t1, 84(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 88(sp)
  lw t0, 88(sp)
  lw t0, 0(t0)
  sw t0, 92(sp)
  la t0, x_1
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 96(sp)
  lw t0, 96(sp)
  lw t0, 0(t0)
  sw t0, 100(sp)
  lw t0, 92(sp)
  lw t1, 100(sp)
  add t0, t0, t1
  sw t0, 104(sp)
  addi t0, sp, 72
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 108(sp)
  lw t0, 108(sp)
  lw t0, 0(t0)
  sw t0, 112(sp)
  lw t0, 104(sp)
  lw t1, 112(sp)
  add t0, t0, t1
  sw t0, 116(sp)
  lw a0, 116(sp)
  addi sp, sp, 128
  ret
 