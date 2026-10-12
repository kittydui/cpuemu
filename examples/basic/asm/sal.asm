bits 64
org 0x1000

; sal is the same operation as shl. NASM encodes `sal` as shl (D1 /4), so the raw bytes
; below use the other encoding (D1 /6), which iced reports as Mnemonic::Sal.
; 3 -> 6 -> 12 = 0b1100 (two 1-bits, PF). No carry, no sign change.
; expect rax = 12
; expect rflags = 0x6

    mov eax, 3
    sal eax, 1
    db 0xD1, 0xF0       ; sal eax, 1 (/6 encoding)
    hlt
