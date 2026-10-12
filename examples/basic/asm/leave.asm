bits 64
org 0x1000

; Build a stack frame, store a local, read it back, and let `leave` tear the frame down.
; expect rax = 42
; expect rsp = 0x8000_0000
; expect rbp = 0

    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov qword [rbp - 8], 42
    mov rax, [rbp - 8]
    leave
    hlt
