bits 64
org 0x1000

; expect rip = 0x1007
; expect executed = 4

    nop
    nop dword [rax+rax]
    nop
    hlt
