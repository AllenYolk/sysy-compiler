  .data
  .globl n_1
n_1:
  .zero 4
 
  .text
  .globl QuickSort
QuickSort:
  addi sp, sp, -384
  sw ra, 380(sp)
entry_1:
  sw a0, 0(sp)
  sw a1, 4(sp)
  sw a2, 8(sp)
  lw t0, 4(sp)
  sw t0, 12(sp)
  lw t0, 8(sp)
  sw t0, 16(sp)
  lw t0, 12(sp)
  lw t1, 16(sp)
  slt t0, t0, t1
  sw t0, 20(sp)
  lw t0, 20(sp)
  bnez t0, if_then_1
  j if_else_1
if_then_1:
  lw t0, 4(sp)
  sw t0, 28(sp)
  lw t0, 28(sp)
  sw t0, 24(sp)
  lw t0, 8(sp)
  sw t0, 36(sp)
  lw t0, 36(sp)
  sw t0, 32(sp)
  lw t0, 4(sp)
  sw t0, 44(sp)
  lw t0, 0(sp)
  sw t0, 48(sp)
  lw t0, 48(sp)
  lw t1, 44(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 52(sp)
  lw t0, 52(sp)
  lw t0, 0(t0)
  sw t0, 56(sp)
  lw t0, 56(sp)
  sw t0, 40(sp)
  j while_entry_1
if_else_1:
  j if_end_1
while_entry_1:
  lw t0, 24(sp)
  sw t0, 60(sp)
  lw t0, 32(sp)
  sw t0, 64(sp)
  lw t0, 60(sp)
  lw t1, 64(sp)
  slt t0, t0, t1
  sw t0, 68(sp)
  lw t0, 68(sp)
  bnez t0, while_body_1
  j while_end_1
if_end_1:
  li a0, 0
  lw ra, 380(sp)
  addi sp, sp, 384
  ret
while_body_1:
  j while_entry_2
while_end_1:
  lw t0, 24(sp)
  sw t0, 72(sp)
  lw t0, 0(sp)
  sw t0, 76(sp)
  lw t0, 76(sp)
  lw t1, 72(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 80(sp)
  lw t0, 40(sp)
  sw t0, 84(sp)
  lw t0, 84(sp)
  lw t1, 80(sp)
  sw t0, 0(t1)
  lw t0, 24(sp)
  sw t0, 92(sp)
  lw t0, 92(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 96(sp)
  lw t0, 96(sp)
  sw t0, 88(sp)
  lw t0, 0(sp)
  sw t0, 100(sp)
  lw t0, 100(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 104(sp)
  lw t0, 4(sp)
  sw t0, 108(sp)
  lw t0, 88(sp)
  sw t0, 112(sp)
  lw a0, 104(sp)
  lw a1, 108(sp)
  lw a2, 112(sp)
  call QuickSort
  sw a0, 116(sp)
  lw t0, 116(sp)
  sw t0, 88(sp)
  lw t0, 24(sp)
  sw t0, 120(sp)
  lw t0, 120(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 124(sp)
  lw t0, 124(sp)
  sw t0, 88(sp)
  lw t0, 0(sp)
  sw t0, 128(sp)
  lw t0, 128(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 132(sp)
  lw t0, 88(sp)
  sw t0, 136(sp)
  lw t0, 8(sp)
  sw t0, 140(sp)
  lw a0, 132(sp)
  lw a1, 136(sp)
  lw a2, 140(sp)
  call QuickSort
  sw a0, 144(sp)
  lw t0, 144(sp)
  sw t0, 88(sp)
  j if_end_1
while_entry_2:
  lw t0, 24(sp)
  sw t0, 152(sp)
  lw t0, 32(sp)
  sw t0, 156(sp)
  lw t0, 152(sp)
  lw t1, 156(sp)
  slt t0, t0, t1
  sw t0, 160(sp)
  lw t0, 160(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 164(sp)
  lw t0, 164(sp)
  sw t0, 148(sp)
  lw t0, 164(sp)
  bnez t0, and_rhs_1
  j and_end_1
and_rhs_1:
  lw t0, 32(sp)
  sw t0, 168(sp)
  lw t0, 0(sp)
  sw t0, 172(sp)
  lw t0, 172(sp)
  lw t1, 168(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 176(sp)
  lw t0, 176(sp)
  lw t0, 0(t0)
  sw t0, 180(sp)
  lw t0, 40(sp)
  sw t0, 184(sp)
  lw t0, 184(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 188(sp)
  lw t0, 180(sp)
  lw t1, 188(sp)
  sgt t0, t0, t1
  sw t0, 192(sp)
  lw t0, 192(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 196(sp)
  lw t0, 196(sp)
  sw t0, 148(sp)
  j and_end_1
and_end_1:
  lw t0, 148(sp)
  sw t0, 200(sp)
  lw t0, 200(sp)
  bnez t0, while_body_2
  j while_end_2
while_body_2:
  lw t0, 32(sp)
  sw t0, 204(sp)
  lw t0, 204(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 208(sp)
  lw t0, 208(sp)
  sw t0, 32(sp)
  j while_entry_2
while_end_2:
  lw t0, 24(sp)
  sw t0, 212(sp)
  lw t0, 32(sp)
  sw t0, 216(sp)
  lw t0, 212(sp)
  lw t1, 216(sp)
  slt t0, t0, t1
  sw t0, 220(sp)
  lw t0, 220(sp)
  bnez t0, if_then_2
  j if_else_2
if_then_2:
  lw t0, 24(sp)
  sw t0, 224(sp)
  lw t0, 0(sp)
  sw t0, 228(sp)
  lw t0, 228(sp)
  lw t1, 224(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 232(sp)
  lw t0, 32(sp)
  sw t0, 236(sp)
  lw t0, 0(sp)
  sw t0, 240(sp)
  lw t0, 240(sp)
  lw t1, 236(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 244(sp)
  lw t0, 244(sp)
  lw t0, 0(t0)
  sw t0, 248(sp)
  lw t0, 248(sp)
  lw t1, 232(sp)
  sw t0, 0(t1)
  lw t0, 24(sp)
  sw t0, 252(sp)
  lw t0, 252(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 256(sp)
  lw t0, 256(sp)
  sw t0, 24(sp)
  j if_end_2
if_else_2:
  j if_end_2
if_end_2:
  j while_entry_3
while_entry_3:
  lw t0, 24(sp)
  sw t0, 264(sp)
  lw t0, 32(sp)
  sw t0, 268(sp)
  lw t0, 264(sp)
  lw t1, 268(sp)
  slt t0, t0, t1
  sw t0, 272(sp)
  lw t0, 272(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 276(sp)
  lw t0, 276(sp)
  sw t0, 260(sp)
  lw t0, 276(sp)
  bnez t0, and_rhs_2
  j and_end_2
and_rhs_2:
  lw t0, 24(sp)
  sw t0, 280(sp)
  lw t0, 0(sp)
  sw t0, 284(sp)
  lw t0, 284(sp)
  lw t1, 280(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 288(sp)
  lw t0, 288(sp)
  lw t0, 0(t0)
  sw t0, 292(sp)
  lw t0, 40(sp)
  sw t0, 296(sp)
  lw t0, 292(sp)
  lw t1, 296(sp)
  slt t0, t0, t1
  sw t0, 300(sp)
  lw t0, 300(sp)
  li t1, 0
  xor t0, t0, t1
  snez t0, t0
  sw t0, 304(sp)
  lw t0, 304(sp)
  sw t0, 260(sp)
  j and_end_2
and_end_2:
  lw t0, 260(sp)
  sw t0, 308(sp)
  lw t0, 308(sp)
  bnez t0, while_body_3
  j while_end_3
while_body_3:
  lw t0, 24(sp)
  sw t0, 312(sp)
  lw t0, 312(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 316(sp)
  lw t0, 316(sp)
  sw t0, 24(sp)
  j while_entry_3
while_end_3:
  lw t0, 24(sp)
  sw t0, 320(sp)
  lw t0, 32(sp)
  sw t0, 324(sp)
  lw t0, 320(sp)
  lw t1, 324(sp)
  slt t0, t0, t1
  sw t0, 328(sp)
  lw t0, 328(sp)
  bnez t0, if_then_3
  j if_else_3
if_then_3:
  lw t0, 32(sp)
  sw t0, 332(sp)
  lw t0, 0(sp)
  sw t0, 336(sp)
  lw t0, 336(sp)
  lw t1, 332(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 340(sp)
  lw t0, 24(sp)
  sw t0, 344(sp)
  lw t0, 0(sp)
  sw t0, 348(sp)
  lw t0, 348(sp)
  lw t1, 344(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 352(sp)
  lw t0, 352(sp)
  lw t0, 0(t0)
  sw t0, 356(sp)
  lw t0, 356(sp)
  lw t1, 340(sp)
  sw t0, 0(t1)
  lw t0, 32(sp)
  sw t0, 360(sp)
  lw t0, 360(sp)
  li t1, 1
  sub t0, t0, t1
  sw t0, 364(sp)
  lw t0, 364(sp)
  sw t0, 32(sp)
  j if_end_3
if_else_3:
  j if_end_3
if_end_3:
  j while_entry_1
 
  .text
  .globl main
main:
  addi sp, sp, -160
  sw ra, 156(sp)
entry_2:
  li t0, 10
  la t1, n_1
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 40(sp)
  li t0, 4
  lw t1, 40(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 44(sp)
  li t0, 3
  lw t1, 44(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 48(sp)
  li t0, 9
  lw t1, 48(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 52(sp)
  li t0, 2
  lw t1, 52(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 56(sp)
  li t0, 0
  lw t1, 56(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 5
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 60(sp)
  li t0, 1
  lw t1, 60(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 6
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 64(sp)
  li t0, 6
  lw t1, 64(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 7
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 68(sp)
  li t0, 5
  lw t1, 68(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 8
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 72(sp)
  li t0, 7
  lw t1, 72(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 9
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 76(sp)
  li t0, 8
  lw t1, 76(sp)
  sw t0, 0(t1)
  li t0, 0
  sw t0, 80(sp)
  li t0, 9
  sw t0, 84(sp)
  addi t0, sp, 0
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 88(sp)
  lw t0, 80(sp)
  sw t0, 92(sp)
  lw t0, 84(sp)
  sw t0, 96(sp)
  lw a0, 88(sp)
  lw a1, 92(sp)
  lw a2, 96(sp)
  call QuickSort
  sw a0, 100(sp)
  lw t0, 100(sp)
  sw t0, 80(sp)
  j while_entry_4
while_entry_4:
  lw t0, 80(sp)
  sw t0, 104(sp)
  la t0, n_1
  lw t0, 0(t0)
  sw t0, 108(sp)
  lw t0, 104(sp)
  lw t1, 108(sp)
  slt t0, t0, t1
  sw t0, 112(sp)
  lw t0, 112(sp)
  bnez t0, while_body_4
  j while_end_4
while_body_4:
  lw t0, 80(sp)
  sw t0, 120(sp)
  addi t0, sp, 0
  lw t1, 120(sp)
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 124(sp)
  lw t0, 124(sp)
  lw t0, 0(t0)
  sw t0, 128(sp)
  lw t0, 128(sp)
  sw t0, 116(sp)
  lw t0, 116(sp)
  sw t0, 132(sp)
  lw a0, 132(sp)
  call putint
  sw a0, 136(sp)
  li t0, 10
  sw t0, 116(sp)
  lw t0, 116(sp)
  sw t0, 140(sp)
  lw a0, 140(sp)
  call putch
  sw a0, 144(sp)
  lw t0, 80(sp)
  sw t0, 148(sp)
  lw t0, 148(sp)
  li t1, 1
  add t0, t0, t1
  sw t0, 152(sp)
  lw t0, 152(sp)
  sw t0, 80(sp)
  j while_entry_4
while_end_4:
  li a0, 0
  lw ra, 156(sp)
  addi sp, sp, 160
  ret
 