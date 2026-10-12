bits 64
org 0x1000

; Last in, first out. The two pushes and two pops leave rsp where it started.
; expect rcx = 5
; expect rdx = 7
; expect rsp = 0x8000_0000

    mov eax, 7
    push rax
    push 5            ; imm8, sign-extended to 64 bits
    pop rcx
    pop rdx
    hlt
