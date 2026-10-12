bits 64
org 0x1000

; The cmp sets CF, AF and SF (5 - 7 borrows). The `and` must overwrite all the
; status flags: CF and OF cleared, AF left clear, SF/ZF/PF from its result 5,
; which has two 1-bits (PF).
; expect rax = 5
; expect rflags = 0x6

    mov rax, 5
    cmp rax, 7
    and rax, 0xff
    hlt
