bits 64
org 0x1000

; not inverts each operand and never touches flags, so the ZF + PF from the cmp
; are still there at the end.
; expect rax = 0xffff_ffff_ffff_fffa
; expect rcx = 0x12cb
; expect rdx = 0
; expect rflags = 0x46

    mov eax, 5
    mov ecx, 0x1234
    mov rdx, -1
    cmp eax, 5        ; ZF + PF
    not rax           ; !5 over 64 bits
    not cl            ; !0x34 = 0xcb, the rest of rcx stays 0x12
    not edx           ; !0xffffffff = 0, and the 32-bit write clears the top half
    hlt