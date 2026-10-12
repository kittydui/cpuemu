bits 64
org 0x1000

; Two-operand form: eax = eax * ecx. 42 fits in 32 bits, so CF = OF = 0 and the jno
; skips setting rbx. SF/ZF/PF are undefined for imul, so rflags isn't checked.
; expect rax = 42
; expect rbx = 0

    mov eax, 6
    mov ecx, 7
    imul eax, ecx
    jno done
    mov ebx, 1
done:
    hlt
