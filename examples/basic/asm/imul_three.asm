bits 64
org 0x1000

; Three-operand form with a sign-extended immediate: eax = ecx * -3 = -21. The 32-bit
; destination clears the upper half of rax. No overflow, so rbx stays 0.
; expect rax = 0xffff_ffeb
; expect rbx = 0

    mov ecx, 7
    imul eax, ecx, -3
    jno done
    mov ebx, 1
done:
    hlt
