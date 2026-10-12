bits 64
org 0x1000

; The usual way compilers zero a register. A 32-bit write clears the upper half too,
; so all of rax ends up 0, not just eax.
; expect rax = 0
; expect rflags = 0x46

    mov rax, -1
    xor eax, eax
    hlt
