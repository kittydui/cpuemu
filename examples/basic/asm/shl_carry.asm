bits 64
org 0x1000

; Shift by cl. For a 4-bit left shift of a 32-bit value, the last bit shifted out is
; bit 28, which is set, so CF = 1. OF is undefined for counts above 1, so only CF is
; checked, through jnc: rbx = 1 means CF was set.
; expect rax = 0x10
; expect rbx = 1

    mov eax, 0x1000_0001
    mov cl, 4
    shl eax, cl
    jnc done
    mov ebx, 1
done:
    hlt
