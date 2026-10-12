bits 64
org 0x1000

; One-operand form: rdx:rax = rax * rcx = -2 * 3 = -6 as a 128-bit value, so rax = -6
; and rdx holds only sign bits. It fits in 64 bits, so CF = OF = 0 and rbx stays 0.
; expect rax = 0xffff_ffff_ffff_fffa
; expect rdx = 0xffff_ffff_ffff_ffff
; expect rbx = 0

    mov rax, -2
    mov rcx, 3
    imul rcx
    jno done
    mov ebx, 1
done:
    hlt
