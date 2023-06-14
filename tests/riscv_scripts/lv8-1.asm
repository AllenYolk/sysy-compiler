  .text
  .globl half_add
half_add:
  addi sp, sp, -32
entry_1:
  sw a0, 0(sp)
  sw a1, 4(sp)
  lw t0, 0(sp)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 2
  div t0, t0, t1
  sw t0, 12(sp)
  lw t0, 4(sp)
  sw t0, 16(sp)
  lw t0, 12(sp)
  lw t1, 16(sp)
  add t0, t0, t1
  sw t0, 20(sp)
  lw a0, 20(sp)
  addi sp, sp, 32
  ret
 
  .text
  .globl ff
ff:
  addi sp, sp, -48
  sw ra, 44(sp)
entry_2:
  sw a0, 0(sp)
  lw t0, 0(sp)
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 2
  rem t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  sw t0, 4(sp)
  lw t0, 0(sp)
  sw t0, 16(sp)
  lw t0, 16(sp)
  li t1, 0
  sgt t0, t0, t1
  sw t0, 20(sp)
  lw t0, 20(sp)
  bnez t0, if_then_1
  j if_else_1
if_then_1:
  li t0, 5
  sw t0, 24(sp)
  j if_end_1
if_else_1:
  lw t0, 0(sp)
  sw t0, 28(sp)
  lw t0, 28(sp)
  li t1, 2
  sub t0, t0, t1
  sw t0, 32(sp)
  lw a0, 32(sp)
  call ff
  sw a0, 36(sp)
  j if_end_1
if_end_1:
  lw ra, 44(sp)
  addi sp, sp, 48
  ret
 
  .text
  .globl gg
gg:
  addi sp, sp, -128
entry_3:
  sw a0, 0(sp)
  sw a1, 4(sp)
  sw a2, 8(sp)
  sw a3, 12(sp)
  sw a4, 16(sp)
  sw a5, 20(sp)
  sw a6, 24(sp)
  sw a7, 28(sp)
  lw t0, 128(sp)
  sw t0, 32(sp)
  lw t0, 132(sp)
  sw t0, 36(sp)
  lw t0, 0(sp)
  sw t0, 44(sp)
  lw t0, 4(sp)
  sw t0, 48(sp)
  lw t0, 44(sp)
  lw t1, 48(sp)
  add t0, t0, t1
  sw t0, 52(sp)
  lw t0, 8(sp)
  sw t0, 56(sp)
  lw t0, 52(sp)
  lw t1, 56(sp)
  add t0, t0, t1
  sw t0, 60(sp)
  lw t0, 12(sp)
  sw t0, 64(sp)
  lw t0, 60(sp)
  lw t1, 64(sp)
  add t0, t0, t1
  sw t0, 68(sp)
  lw t0, 16(sp)
  sw t0, 72(sp)
  lw t0, 68(sp)
  lw t1, 72(sp)
  add t0, t0, t1
  sw t0, 76(sp)
  lw t0, 20(sp)
  sw t0, 80(sp)
  lw t0, 76(sp)
  lw t1, 80(sp)
  add t0, t0, t1
  sw t0, 84(sp)
  lw t0, 24(sp)
  sw t0, 88(sp)
  lw t0, 84(sp)
  lw t1, 88(sp)
  add t0, t0, t1
  sw t0, 92(sp)
  lw t0, 28(sp)
  sw t0, 96(sp)
  lw t0, 92(sp)
  lw t1, 96(sp)
  add t0, t0, t1
  sw t0, 100(sp)
  lw t0, 32(sp)
  sw t0, 104(sp)
  lw t0, 100(sp)
  lw t1, 104(sp)
  add t0, t0, t1
  sw t0, 108(sp)
  lw t0, 36(sp)
  sw t0, 112(sp)
  lw t0, 108(sp)
  lw t1, 112(sp)
  add t0, t0, t1
  sw t0, 116(sp)
  lw t0, 116(sp)
  sw t0, 40(sp)
  lw t0, 40(sp)
  sw t0, 120(sp)
  lw a0, 120(sp)
  addi sp, sp, 128
  ret
 
  .text
  .globl main
main:
  addi sp, sp, -32
  sw ra, 28(sp)
entry_4:
  li t0, 5
  li t1, 2
  mul t0, t0, t1
  sw t0, 0(sp)
  li t0, 3
  lw t1, 0(sp)
  add t0, t0, t1
  sw t0, 4(sp)
  lw a0, 4(sp)
  call ff
  sw a0, 8(sp)
  li a0, 10
  li a1, 1
  call half_add
  sw a0, 12(sp)
  lw a0, 12(sp)
  lw ra, 28(sp)
  addi sp, sp, 32
  ret
 