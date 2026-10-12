bits 64
org 0x1000

; 1 + 2 inside the function, then + 10 after returning.
; expect rax = 13
; expect rsp = 0x8000_0000
; expect executed = 6

    mov eax, 1
    call add_two
    add eax, 10
    hlt

add_two:
    add eax, 2
    ret
