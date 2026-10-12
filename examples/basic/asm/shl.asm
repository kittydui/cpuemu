bits 64
org 0x1000

; 1011_0110 << 1 = 0110_1100. The top bit shifted out sets CF. For a 1-bit shift
; OF = top bit of the result XOR CF = 0 XOR 1 = 1 (the sign changed). Four 1-bits (PF).
; This uses the implicit-1 encoding (D0 /4).
; expect rax = 0x6c
; expect rflags = 0x807

    mov al, 0b1011_0110
    shl al, 1
    hlt
