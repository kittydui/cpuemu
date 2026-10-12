bits 64
org 0x1000

; The same input as shr.asm, but sar fills with copies of the sign bit, so the result
; stays negative (SF). CF = the bit shifted out (1). OF is always 0 for sar.
; expect rax = 0xc000_0001
; expect rflags = 0x83

    mov eax, 0x8000_0003
    sar eax, 1
    hlt
