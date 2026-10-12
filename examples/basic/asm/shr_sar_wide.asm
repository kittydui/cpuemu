bits 64
org 0x1000

; Multi-bit 64-bit shifts of -64: sar divides as signed (-64 / 8 = -8), while shr
; treats it as unsigned and leaves only the top 4 bits.
; expect rax = 0xffff_ffff_ffff_fff8
; expect rdx = 0xf

    mov rax, -64
    sar rax, 3
    mov rdx, -64
    shr rdx, 60
    hlt
