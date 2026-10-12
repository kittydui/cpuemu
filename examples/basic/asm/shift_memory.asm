bits 64
org 0x1000

; Shifting a memory operand in place: 3 << 2 = 12, read back through rax.
; expect rax = 12

    mov qword [rsp - 8], 3
    shl qword [rsp - 8], 2
    mov rax, [rsp - 8]
    hlt
