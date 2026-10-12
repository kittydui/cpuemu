bits 64
org 0x1000

; Indirect jmp through memory, like a switch table: the target address is read
; from `table` with a RIP-relative operand.
; expect rcx = 1
; expect executed = 3

    jmp [rel table]
    mov ecx, 2
target:
    mov ecx, 1
    hlt

table:
    dq target
