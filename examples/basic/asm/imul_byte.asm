bits 64
org 0x1000

; One-operand 8-bit form: ax = al * cl = 100 * 3 = 300 = 0x012C, so al = 0x2C and
; ah = 0x01. 300 doesn't fit in a signed byte, so CF = OF = 1 and rbx is set.
; expect rax = 0x12c
; expect rbx = 1

    mov al, 100
    mov cl, 3
    imul cl
    jno done
    mov ebx, 1
done:
    hlt
