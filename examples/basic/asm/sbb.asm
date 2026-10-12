bits 64
org 0x1000

; sbb subtracts CF as well: the same 10 - 3 gives 7 with CF clear and 6 with CF set.
; expect rax = 7
; expect rcx = 6

    xor eax, eax        ; CF = 0
    mov eax, 10
    sbb eax, 3          ; 10 - 3 - 0
    mov ecx, 5
    cmp ecx, 7          ; CF = 1
    mov ecx, 10
    sbb ecx, 3          ; 10 - 3 - 1
    hlt
