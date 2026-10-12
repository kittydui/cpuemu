bits 64
org 0x1000

; 5 - 7 borrows (CF, so `jb` would jump) and is negative (SF).
; The low byte 0xFE has seven 1-bits, so PF is clear. 5 - 7 also borrows out of bit 3 (AF).
; Neither register changes.
; expect rax = 5
; expect rcx = 7
; expect rflags = 0x93
; expect rip = 0x100e
; expect executed = 4

    mov rax, 5
    mov rcx, 7
    cmp rax, rcx
    hlt
