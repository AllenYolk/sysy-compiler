  .text
  .globl f
f:
  addi sp, sp, -32
entry_1:
  sw a0, 0(sp)
  lw t0, 0(sp)
  sw t0, 4(sp)
  lw t0, 4(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 8(sp)
  lw t0, 8(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 12(sp)
  lw t0, 12(sp)
  lw t0, 0(t0)
  sw t0, 16(sp)
  lw a0, 16(sp)
  addi sp, sp, 32
  ret
 
  .text
  .globl main
main:
  addi sp, sp, -496
  sw ra, 492(sp)
entry_2:
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 120(sp)
  lw t0, 120(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 124(sp)
  lw t0, 124(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 128(sp)
  li t0, 0
  lw t1, 128(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 132(sp)
  lw t0, 132(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 136(sp)
  lw t0, 136(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 140(sp)
  li t0, 1
  lw t1, 140(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 144(sp)
  lw t0, 144(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 148(sp)
  lw t0, 148(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 152(sp)
  li t0, 2
  lw t1, 152(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 156(sp)
  lw t0, 156(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 160(sp)
  lw t0, 160(sp)
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 164(sp)
  li t0, 3
  lw t1, 164(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 168(sp)
  lw t0, 168(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 172(sp)
  lw t0, 172(sp)
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 176(sp)
  li t0, 4
  lw t1, 176(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 180(sp)
  lw t0, 180(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 184(sp)
  lw t0, 184(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 188(sp)
  li t0, 5
  lw t1, 188(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 192(sp)
  lw t0, 192(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 196(sp)
  lw t0, 196(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 200(sp)
  li t0, 6
  lw t1, 200(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 204(sp)
  lw t0, 204(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 208(sp)
  lw t0, 208(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 212(sp)
  li t0, 0
  lw t1, 212(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 216(sp)
  lw t0, 216(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 220(sp)
  lw t0, 220(sp)
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 224(sp)
  li t0, 0
  lw t1, 224(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 228(sp)
  lw t0, 228(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 232(sp)
  lw t0, 232(sp)
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 236(sp)
  li t0, 0
  lw t1, 236(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 240(sp)
  lw t0, 240(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 244(sp)
  lw t0, 244(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 248(sp)
  li t0, 7
  lw t1, 248(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 252(sp)
  lw t0, 252(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 256(sp)
  lw t0, 256(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 260(sp)
  li t0, 0
  lw t1, 260(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 264(sp)
  lw t0, 264(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 268(sp)
  lw t0, 268(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 272(sp)
  li t0, 0
  lw t1, 272(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 276(sp)
  lw t0, 276(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 280(sp)
  lw t0, 280(sp)
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 284(sp)
  li t0, 0
  lw t1, 284(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 288(sp)
  lw t0, 288(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 292(sp)
  lw t0, 292(sp)
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 296(sp)
  li t0, 0
  lw t1, 296(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 300(sp)
  lw t0, 300(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 304(sp)
  lw t0, 304(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 308(sp)
  li t0, 0
  lw t1, 308(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 312(sp)
  lw t0, 312(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 316(sp)
  lw t0, 316(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 320(sp)
  li t0, 0
  lw t1, 320(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 324(sp)
  lw t0, 324(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 328(sp)
  lw t0, 328(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 332(sp)
  li t0, 0
  lw t1, 332(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 336(sp)
  lw t0, 336(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 340(sp)
  lw t0, 340(sp)
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 344(sp)
  li t0, 0
  lw t1, 344(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 348(sp)
  lw t0, 348(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 352(sp)
  lw t0, 352(sp)
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 356(sp)
  li t0, 0
  lw t1, 356(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 360(sp)
  lw t0, 360(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 364(sp)
  lw t0, 364(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 368(sp)
  li t0, 0
  lw t1, 368(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 372(sp)
  lw t0, 372(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 376(sp)
  lw t0, 376(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 380(sp)
  li t0, 0
  lw t1, 380(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 384(sp)
  lw t0, 384(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 388(sp)
  lw t0, 388(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 392(sp)
  li t0, 0
  lw t1, 392(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 396(sp)
  lw t0, 396(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 400(sp)
  lw t0, 400(sp)
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 404(sp)
  li t0, 0
  lw t1, 404(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 408(sp)
  lw t0, 408(sp)
  li t1, 1
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 412(sp)
  lw t0, 412(sp)
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 416(sp)
  li t0, 0
  lw t1, 416(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 420(sp)
  lw t0, 420(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 424(sp)
  lw t0, 424(sp)
  li t1, 0
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 428(sp)
  li t0, 0
  lw t1, 428(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 432(sp)
  lw t0, 432(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 436(sp)
  lw t0, 436(sp)
  li t1, 1
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 440(sp)
  li t0, 0
  lw t1, 440(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 444(sp)
  lw t0, 444(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 448(sp)
  lw t0, 448(sp)
  li t1, 2
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 452(sp)
  li t0, 0
  lw t1, 452(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 456(sp)
  lw t0, 456(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 460(sp)
  lw t0, 460(sp)
  li t1, 3
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 464(sp)
  li t0, 0
  lw t1, 464(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 1
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 468(sp)
  lw t0, 468(sp)
  li t1, 2
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 472(sp)
  lw t0, 472(sp)
  li t1, 4
  li t2, 4
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 476(sp)
  li t0, 0
  lw t1, 476(sp)
  sw t0, 0(t1)
  addi t0, sp, 0
  li t1, 0
  li t2, 60
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 480(sp)
  lw t0, 480(sp)
  li t1, 0
  li t2, 20
  mul t1, t1, t2
  add t0, t0, t1
  sw t0, 484(sp)
  lw a0, 484(sp)
  call f
  sw a0, 488(sp)
  lw a0, 488(sp)
  lw ra, 492(sp)
  addi sp, sp, 496
  ret
 