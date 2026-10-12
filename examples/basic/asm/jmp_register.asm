bits 64
org 0x1000

; Indirect jmp through a register: rax holds the target's address.
; expect rcx = 1
; expect executed = 4

    mov eax, target
    jmp rax
    mov ecx, 2
target:
    mov ecx, 1
    hlt
