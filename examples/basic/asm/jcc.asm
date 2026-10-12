bits 64
org 0x1000

; Each check sets one bit of rbx only if its branch went the right way, so
; rbx = 0x7ff means all 11 checks passed. Each check redoes its cmp/test, because
; the `or` that records a result changes the flags.
; expect rbx = 0x7ff

%macro taken 2              ; jcc, bit: the jump has to be taken
    %1 %%yes
    jmp %%done
%%yes:
    or ebx, 1 << %2
%%done:
%endmacro

%macro not_taken 2          ; jcc, bit: the jump has to fall through
    %1 %%skip
    or ebx, 1 << %2
%%skip:
%endmacro

    xor ebx, ebx

    ; 5 == 5
    mov eax, 5
    cmp eax, 5
    taken je, 0
    cmp eax, 5
    not_taken jne, 1

    ; -1 vs 1: above as unsigned, less as signed
    mov eax, -1
    cmp eax, 1
    taken ja, 2
    cmp eax, 1
    not_taken jb, 3
    cmp eax, 1
    taken jl, 4
    cmp eax, 1
    not_taken jg, 5

    ; INT_MIN vs 1: signed overflow (OF set, SF clear), but still less as signed
    mov eax, 0x8000_0000
    cmp eax, 1
    taken jl, 6
    cmp eax, 1
    taken jo, 7
    cmp eax, 1
    not_taken jge, 8

    ; 3 has two 1-bits (even parity) and is positive
    mov eax, 3
    test eax, eax
    taken jp, 9
    test eax, eax
    not_taken js, 10

    hlt
