bits 64          ; -f bin defaults to 16-bit, so this is required
org 0x1000       ; the address you load it at, so absolute addresses come out right

nop
nop dword [rax+rax]
nop
hlt