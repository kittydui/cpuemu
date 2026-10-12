bits 64
org 0x1000

; A backwards jnz loop: 10 + 9 + ... + 1 = 55. The last `sub ecx, 1` gives 0 (ZF + PF),
; so the jnz falls through. executed = 2 setup + 10 loops x 3 + hlt = 33.
; expect rax = 55
; expect rcx = 0
; expect rflags = 0x46
; expect rip = 0x100f
; expect executed = 33

    xor eax, eax
    mov ecx, 10
next:
    add eax, ecx
    sub ecx, 1
    jnz next
    hlt
