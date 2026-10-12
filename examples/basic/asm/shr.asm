bits 64
org 0x1000

; shr fills with zeros, so the negative 32-bit value becomes positive.
; CF = the bit shifted out (bit 0 = 1). For a 1-bit shift OF = the original top bit (1).
; The low byte 0x01 has one 1-bit, so PF is clear.
; expect rax = 0x4000_0001
; expect rflags = 0x803

    mov eax, 0x8000_0003
    shr eax, 1
    hlt
