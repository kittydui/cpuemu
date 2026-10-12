bits 64
org 0x1000

; -(-128) doesn't fit in a signed byte, so the result is -128 again with OF set.
; CF (source wasn't 0) and SF (negative) are set too. 0x80 has one 1-bit, so PF is clear.
; expect rax = 0x80
; expect rflags = 0x883

    mov al, 0x80
    neg al
    hlt
