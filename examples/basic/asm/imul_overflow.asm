bits 64
org 0x1000

; 0x10000 * 0x10000 = 2^32 doesn't fit in 32 bits: the result is truncated to 0 and
; CF = OF = 1, so jno falls through and sets rbx.
; expect rax = 0
; expect rbx = 1

    mov eax, 0x10000
    imul eax, eax
    jno done
    mov ebx, 1
done:
    hlt
