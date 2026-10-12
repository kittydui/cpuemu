bits 64
org 0x1000

; adc adds CF in: the same 2 + 3 gives 5 with CF clear and 6 with CF set.
; expect rax = 5
; expect rcx = 6

    xor eax, eax        ; CF = 0
    mov eax, 2
    adc eax, 3          ; 2 + 3 + 0
    mov ecx, 5
    cmp ecx, 7          ; 5 - 7 borrows: CF = 1
    mov ecx, 2
    adc ecx, 3          ; 2 + 3 + 1
    hlt
