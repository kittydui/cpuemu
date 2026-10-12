bits 64
org 0x1000

; test is an `and` that only sets flags. 0b1010 & 0b0101 = 0 sets ZF and PF,
; and rax keeps its value.
; expect rax = 10
; expect rflags = 0x46
; expect executed = 3

    mov rax, 0b1010
    test rax, 0b0101
    hlt
