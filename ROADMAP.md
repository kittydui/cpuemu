# x86-64

Instruction support in `emu-arch-x86`. An instruction is ticked once it's dispatched in `exec/mod.rs` and has a test in `examples/basic/asm/`. Click a section to expand it.

## Common Instructions

**26 / 129 done**

<details>
<summary>Data transfer: <code>data.rs</code> (2/13)</summary>

- [x] `mov`
- [ ] `movzx`
- [ ] `movsx`
- [ ] `movsxd`
- [x] `lea`
- [ ] `xchg`
- [ ] `cmovcc` (all 16 conditions)
- [ ] `bswap`
- [ ] `xadd`
- [ ] `cmpxchg`
- [ ] `cmpxchg8b`
- [ ] `cmpxchg16b`
- [ ] `xlat`

</details>

<details>
<summary>Arithmetic: <code>arith.rs</code> (6/8)</summary>

- [x] `add`
- [ ] `adc`
- [x] `sub`
- [ ] `sbb`
- [x] `cmp`
- [x] `inc`
- [x] `dec`
- [x] `neg`

</details>

<details>
<summary>Multiply and divide: <code>mul.rs</code> (1/4)</summary>

- [ ] `mul`
- [x] `imul` (1-, 2- and 3-operand forms)
- [ ] `div`
- [ ] `idiv`

</details>

<details>
<summary>Logic: <code>logic.rs</code> (5/5)</summary>

- [x] `and`
- [x] `or`
- [x] `xor`
- [x] `not`
- [x] `test`

</details>

<details>
<summary>Shift and rotate: <code>shift.rs</code> (3/9)</summary>

- [x] `shl` / `sal`
- [x] `shr`
- [x] `sar`
- [ ] `rol`
- [ ] `ror`
- [ ] `rcl`
- [ ] `rcr`
- [ ] `shld`
- [ ] `shrd`

</details>

<details>
<summary>Bit operations: <code>bit.rs</code> (0/6)</summary>

- [ ] `bt`
- [ ] `bts`
- [ ] `btr`
- [ ] `btc`
- [ ] `bsf`
- [ ] `bsr`

</details>

<details>
<summary>Control flow: <code>control.rs</code> (4/11)</summary>

- [x] `jmp`
- [x] `jcc` (all 16 conditions)
- [x] `call`
- [x] `ret` / `ret n`
- [ ] `setcc` (all 16 conditions)
- [ ] `loop`
- [ ] `loope` / `loopz`
- [ ] `loopne` / `loopnz`
- [ ] `jrcxz`
- [ ] `int n`
- [ ] `int3`

</details>

<details>
<summary>Stack: <code>stack.rs</code> (3/6)</summary>

- [x] `push`
- [x] `pop`
- [ ] `enter`
- [x] `leave`
- [ ] `pushf`
- [ ] `popf`

</details>

<details>
<summary>Conversion: <code>conv.rs</code> (0/6)</summary>

- [ ] `cbw`
- [ ] `cwde`
- [ ] `cdqe`
- [ ] `cwd`
- [ ] `cdq`
- [ ] `cqo`

</details>

<details>
<summary>String: <code>string.rs</code> (0/8)</summary>

- [ ] `movs`
- [ ] `cmps`
- [ ] `scas`
- [ ] `lods`
- [ ] `stos`
- [ ] `rep` prefix
- [ ] `repe` / `repz` prefix
- [ ] `repne` / `repnz` prefix

</details>

<details>
<summary>Flags: <code>flag.rs</code> (0/9)</summary>

- [ ] `clc`
- [ ] `stc`
- [ ] `cmc`
- [ ] `cld`
- [ ] `std`
- [ ] `lahf`
- [ ] `sahf`
- [ ] `cli`
- [ ] `sti`

</details>

<details>
<summary>Miscellaneous: <code>misc.rs</code> (2/11)</summary>

- [x] `nop` (including multi-byte `nop r/m`)
- [x] `hlt`
- [ ] `cpuid`
- [ ] `rdtsc`
- [ ] `rdtscp`
- [ ] `pause`
- [ ] `ud2`
- [ ] `lfence`
- [ ] `mfence`
- [ ] `sfence`
- [ ] `clflush`

</details>

<details>
<summary>Port I/O (0/4)</summary>

- [ ] `in`
- [ ] `out`
- [ ] `ins`
- [ ] `outs`

</details>

<details>
<summary>System (privileged) (0/29)</summary>

- [ ] `mov` to/from segment registers
- [ ] `mov` to/from control registers (`cr0`–`cr8`)
- [ ] `mov` to/from debug registers (`dr0`–`dr7`)
- [ ] `lgdt`
- [ ] `sgdt`
- [ ] `lidt`
- [ ] `sidt`
- [ ] `lldt`
- [ ] `sldt`
- [ ] `ltr`
- [ ] `str`
- [ ] `lar`
- [ ] `lsl`
- [ ] `verr`
- [ ] `verw`
- [ ] `rdmsr`
- [ ] `wrmsr`
- [ ] `invlpg`
- [ ] `swapgs`
- [ ] `syscall`
- [ ] `sysret`
- [ ] `sysenter`
- [ ] `sysexit`
- [ ] `iretq`
- [ ] `wbinvd`
- [ ] `invd`
- [ ] `clts`
- [ ] `lmsw`
- [ ] `smsw`

</details>

## Extensions

SSE and SSE2 are part of baseline x86-64, so compilers use them for floating point and `memcpy` even without any `-m` flags. Those two are the ones to do first.

iced reports the same mnemonic for some instructions that differ only by encoding. For example, `movsd` is both the SSE2 scalar move and the string instruction, and MMX and SSE2 share `paddb` and friends. Dispatch on `instruction.code()` or the operand kinds where that matters.

<details>
<summary>SSE (0/50)</summary>

- [ ] `movaps`
- [ ] `movups`
- [ ] `movss`
- [ ] `movlps`
- [ ] `movhps`
- [ ] `movlhps`
- [ ] `movhlps`
- [ ] `movmskps`
- [ ] `addps`
- [ ] `addss`
- [ ] `subps`
- [ ] `subss`
- [ ] `mulps`
- [ ] `mulss`
- [ ] `divps`
- [ ] `divss`
- [ ] `sqrtps`
- [ ] `sqrtss`
- [ ] `rcpps`
- [ ] `rcpss`
- [ ] `rsqrtps`
- [ ] `rsqrtss`
- [ ] `maxps`
- [ ] `maxss`
- [ ] `minps`
- [ ] `minss`
- [ ] `cmpps`
- [ ] `cmpss`
- [ ] `comiss`
- [ ] `ucomiss`
- [ ] `andps`
- [ ] `andnps`
- [ ] `orps`
- [ ] `xorps`
- [ ] `shufps`
- [ ] `unpcklps`
- [ ] `unpckhps`
- [ ] `cvtsi2ss`
- [ ] `cvtss2si`
- [ ] `cvttss2si`
- [ ] `cvtpi2ps`
- [ ] `cvtps2pi`
- [ ] `cvttps2pi`
- [ ] `ldmxcsr`
- [ ] `stmxcsr`
- [ ] `movntps`
- [ ] `prefetcht0`
- [ ] `prefetcht1`
- [ ] `prefetcht2`
- [ ] `prefetchnta`

</details>

<details>
<summary>SSE2 (0/117)</summary>

- [ ] `movapd`
- [ ] `movupd`
- [ ] `movsd`
- [ ] `movlpd`
- [ ] `movhpd`
- [ ] `movmskpd`
- [ ] `movdqa`
- [ ] `movdqu`
- [ ] `movd`
- [ ] `movq`
- [ ] `movntpd`
- [ ] `movntdq`
- [ ] `movnti`
- [ ] `maskmovdqu`
- [ ] `addpd`
- [ ] `addsd`
- [ ] `subpd`
- [ ] `subsd`
- [ ] `mulpd`
- [ ] `mulsd`
- [ ] `divpd`
- [ ] `divsd`
- [ ] `sqrtpd`
- [ ] `sqrtsd`
- [ ] `maxpd`
- [ ] `maxsd`
- [ ] `minpd`
- [ ] `minsd`
- [ ] `cmppd`
- [ ] `cmpsd`
- [ ] `comisd`
- [ ] `ucomisd`
- [ ] `andpd`
- [ ] `andnpd`
- [ ] `orpd`
- [ ] `xorpd`
- [ ] `shufpd`
- [ ] `unpcklpd`
- [ ] `unpckhpd`
- [ ] `cvtsi2sd`
- [ ] `cvtsd2si`
- [ ] `cvttsd2si`
- [ ] `cvtss2sd`
- [ ] `cvtsd2ss`
- [ ] `cvtps2pd`
- [ ] `cvtpd2ps`
- [ ] `cvtdq2ps`
- [ ] `cvtps2dq`
- [ ] `cvttps2dq`
- [ ] `cvtdq2pd`
- [ ] `cvtpd2dq`
- [ ] `cvttpd2dq`
- [ ] `paddb`
- [ ] `paddw`
- [ ] `paddd`
- [ ] `paddq`
- [ ] `psubb`
- [ ] `psubw`
- [ ] `psubd`
- [ ] `psubq`
- [ ] `paddsb`
- [ ] `paddsw`
- [ ] `paddusb`
- [ ] `paddusw`
- [ ] `psubsb`
- [ ] `psubsw`
- [ ] `psubusb`
- [ ] `psubusw`
- [ ] `pmullw`
- [ ] `pmulhw`
- [ ] `pmulhuw`
- [ ] `pmuludq`
- [ ] `pmaddwd`
- [ ] `pavgb`
- [ ] `pavgw`
- [ ] `pminub`
- [ ] `pmaxub`
- [ ] `pminsw`
- [ ] `pmaxsw`
- [ ] `psadbw`
- [ ] `pand`
- [ ] `pandn`
- [ ] `por`
- [ ] `pxor`
- [ ] `pcmpeqb`
- [ ] `pcmpeqw`
- [ ] `pcmpeqd`
- [ ] `pcmpgtb`
- [ ] `pcmpgtw`
- [ ] `pcmpgtd`
- [ ] `psllw`
- [ ] `pslld`
- [ ] `psllq`
- [ ] `psrlw`
- [ ] `psrld`
- [ ] `psrlq`
- [ ] `psraw`
- [ ] `psrad`
- [ ] `pslldq`
- [ ] `psrldq`
- [ ] `pshufd`
- [ ] `pshuflw`
- [ ] `pshufhw`
- [ ] `punpcklbw`
- [ ] `punpcklwd`
- [ ] `punpckldq`
- [ ] `punpcklqdq`
- [ ] `punpckhbw`
- [ ] `punpckhwd`
- [ ] `punpckhdq`
- [ ] `punpckhqdq`
- [ ] `packsswb`
- [ ] `packssdw`
- [ ] `packuswb`
- [ ] `pmovmskb`
- [ ] `pextrw`
- [ ] `pinsrw`

</details>

<details>
<summary>SSE3 (0/13)</summary>

- [ ] `addsubps`
- [ ] `addsubpd`
- [ ] `haddps`
- [ ] `haddpd`
- [ ] `hsubps`
- [ ] `hsubpd`
- [ ] `movddup`
- [ ] `movshdup`
- [ ] `movsldup`
- [ ] `lddqu`
- [ ] `fisttp`
- [ ] `monitor`
- [ ] `mwait`

</details>

<details>
<summary>SSSE3 (0/16)</summary>

- [ ] `pshufb`
- [ ] `phaddw`
- [ ] `phaddd`
- [ ] `phaddsw`
- [ ] `phsubw`
- [ ] `phsubd`
- [ ] `phsubsw`
- [ ] `pmaddubsw`
- [ ] `pmulhrsw`
- [ ] `psignb`
- [ ] `psignw`
- [ ] `psignd`
- [ ] `pabsb`
- [ ] `pabsw`
- [ ] `pabsd`
- [ ] `palignr`

</details>

<details>
<summary>SSE4.1 (0/48)</summary>

- [ ] `blendps`
- [ ] `blendpd`
- [ ] `blendvps`
- [ ] `blendvpd`
- [ ] `pblendvb`
- [ ] `pblendw`
- [ ] `dpps`
- [ ] `dppd`
- [ ] `insertps`
- [ ] `extractps`
- [ ] `pextrb`
- [ ] `pextrd`
- [ ] `pextrq`
- [ ] `pinsrb`
- [ ] `pinsrd`
- [ ] `pinsrq`
- [ ] `pminsb`
- [ ] `pminsd`
- [ ] `pminuw`
- [ ] `pminud`
- [ ] `pmaxsb`
- [ ] `pmaxsd`
- [ ] `pmaxuw`
- [ ] `pmaxud`
- [ ] `pmulld`
- [ ] `pmuldq`
- [ ] `pcmpeqq`
- [ ] `packusdw`
- [ ] `ptest`
- [ ] `roundps`
- [ ] `roundpd`
- [ ] `roundss`
- [ ] `roundsd`
- [ ] `mpsadbw`
- [ ] `phminposuw`
- [ ] `movntdqa`
- [ ] `pmovsxbw`
- [ ] `pmovsxbd`
- [ ] `pmovsxbq`
- [ ] `pmovsxwd`
- [ ] `pmovsxwq`
- [ ] `pmovsxdq`
- [ ] `pmovzxbw`
- [ ] `pmovzxbd`
- [ ] `pmovzxbq`
- [ ] `pmovzxwd`
- [ ] `pmovzxwq`
- [ ] `pmovzxdq`

</details>

<details>
<summary>SSE4.2 (0/6)</summary>

- [ ] `pcmpestri`
- [ ] `pcmpestrm`
- [ ] `pcmpistri`
- [ ] `pcmpistrm`
- [ ] `pcmpgtq`
- [ ] `crc32`

</details>

<details>
<summary>AVX (0/16)</summary>

- [ ] VEX-encoded (`v`-prefixed) 128-bit forms of the SSE–SSE4.2 instructions
- [ ] 256-bit `ymm` forms of the floating-point instructions
- [ ] `vbroadcastss`
- [ ] `vbroadcastsd`
- [ ] `vbroadcastf128`
- [ ] `vinsertf128`
- [ ] `vextractf128`
- [ ] `vperm2f128`
- [ ] `vpermilps`
- [ ] `vpermilpd`
- [ ] `vmaskmovps`
- [ ] `vmaskmovpd`
- [ ] `vtestps`
- [ ] `vtestpd`
- [ ] `vzeroupper`
- [ ] `vzeroall`

</details>

<details>
<summary>AVX2 (0/29)</summary>

- [ ] 256-bit `ymm` forms of the SSE2–SSE4.1 integer instructions
- [ ] `vpbroadcastb`
- [ ] `vpbroadcastw`
- [ ] `vpbroadcastd`
- [ ] `vpbroadcastq`
- [ ] `vbroadcasti128`
- [ ] `vinserti128`
- [ ] `vextracti128`
- [ ] `vperm2i128`
- [ ] `vpermd`
- [ ] `vpermps`
- [ ] `vpermq`
- [ ] `vpermpd`
- [ ] `vpmaskmovd`
- [ ] `vpmaskmovq`
- [ ] `vpsllvd`
- [ ] `vpsllvq`
- [ ] `vpsrlvd`
- [ ] `vpsrlvq`
- [ ] `vpsravd`
- [ ] `vpblendd`
- [ ] `vgatherdps`
- [ ] `vgatherdpd`
- [ ] `vgatherqps`
- [ ] `vgatherqpd`
- [ ] `vpgatherdd`
- [ ] `vpgatherdq`
- [ ] `vpgatherqd`
- [ ] `vpgatherqq`

</details>

<details>
<summary>FMA3 (0/6)</summary>

- [ ] `vfmadd{132,213,231}{ps,pd,ss,sd}`
- [ ] `vfmsub{132,213,231}{ps,pd,ss,sd}`
- [ ] `vfnmadd{132,213,231}{ps,pd,ss,sd}`
- [ ] `vfnmsub{132,213,231}{ps,pd,ss,sd}`
- [ ] `vfmaddsub{132,213,231}{ps,pd}`
- [ ] `vfmsubadd{132,213,231}{ps,pd}`

</details>

<details>
<summary>AVX-512 (0/15)</summary>

- [ ] AVX-512F: EVEX-encoded 512-bit forms (`zmm0`–`zmm31`) of the AVX/AVX2 instructions
- [ ] Opmask registers `k0`–`k7`: merge/zero masking, embedded broadcast, embedded rounding
- [ ] Opmask instructions: `kand`, `kandn`, `kor`, `kxor`, `kxnor`, `knot`, `kmov`, `kshiftl`, `kshiftr`, `kortest`, `ktest`, `kadd`, `kunpck`
- [ ] `vpternlogd` / `vpternlogq`
- [ ] Compress/expand: `vpcompressd/q`, `vcompressps/pd`, `vpexpandd/q`, `vexpandps/pd`
- [ ] Two-table permutes: `vpermt2*` / `vpermi2*`
- [ ] Scatters: `vpscatter*` / `vscatter*`
- [ ] `vrcp14*` / `vrsqrt14*`
- [ ] `vgetexp*`, `vgetmant*`, `vscalef*`, `vfixupimm*`, `vrndscale*`
- [ ] Down-converting moves: `vpmov*` (with saturating variants)
- [ ] AVX-512CD: `vpconflictd/q`, `vplzcntd/q`, `vpbroadcastm*`
- [ ] AVX-512BW: byte/word forms
- [ ] AVX-512DQ: dword/qword forms, `vfpclass*`, `vrange*`, `vreduce*`, 64-bit integer conversions
- [ ] AVX-512VL: 128/256-bit EVEX forms
- [ ] Later subsets: VBMI, VBMI2, VNNI, BITALG, VPOPCNTDQ, IFMA, BF16, FP16

</details>

<details>
<summary>x87 FPU (0/81)</summary>

- [ ] `fld`
- [ ] `fst`
- [ ] `fstp`
- [ ] `fild`
- [ ] `fist`
- [ ] `fistp`
- [ ] `fbld`
- [ ] `fbstp`
- [ ] `fxch`
- [ ] `fld1`
- [ ] `fldz`
- [ ] `fldpi`
- [ ] `fldl2e`
- [ ] `fldl2t`
- [ ] `fldlg2`
- [ ] `fldln2`
- [ ] `fadd`
- [ ] `faddp`
- [ ] `fiadd`
- [ ] `fsub`
- [ ] `fsubp`
- [ ] `fisub`
- [ ] `fsubr`
- [ ] `fsubrp`
- [ ] `fisubr`
- [ ] `fmul`
- [ ] `fmulp`
- [ ] `fimul`
- [ ] `fdiv`
- [ ] `fdivp`
- [ ] `fidiv`
- [ ] `fdivr`
- [ ] `fdivrp`
- [ ] `fidivr`
- [ ] `fchs`
- [ ] `fabs`
- [ ] `fsqrt`
- [ ] `fprem`
- [ ] `fprem1`
- [ ] `frndint`
- [ ] `fscale`
- [ ] `fxtract`
- [ ] `fsin`
- [ ] `fcos`
- [ ] `fsincos`
- [ ] `fptan`
- [ ] `fpatan`
- [ ] `f2xm1`
- [ ] `fyl2x`
- [ ] `fyl2xp1`
- [ ] `fcom`
- [ ] `fcomp`
- [ ] `fcompp`
- [ ] `fucom`
- [ ] `fucomp`
- [ ] `fucompp`
- [ ] `ficom`
- [ ] `ficomp`
- [ ] `fcomi`
- [ ] `fcomip`
- [ ] `fucomi`
- [ ] `fucomip`
- [ ] `fcmovcc`
- [ ] `ftst`
- [ ] `fxam`
- [ ] `finit` / `fninit`
- [ ] `fldcw`
- [ ] `fstcw` / `fnstcw`
- [ ] `fstsw` / `fnstsw`
- [ ] `fclex` / `fnclex`
- [ ] `fldenv`
- [ ] `fstenv` / `fnstenv`
- [ ] `fsave` / `fnsave`
- [ ] `frstor`
- [ ] `ffree`
- [ ] `fincstp`
- [ ] `fdecstp`
- [ ] `fnop`
- [ ] `fwait` / `wait`
- [ ] `fxsave`
- [ ] `fxrstor`

</details>

<details>
<summary>MMX (0/4)</summary>

- [ ] `emms`
- [ ] `movd` (`mm` registers)
- [ ] `movq` (`mm` registers)
- [ ] `mm0`–`mm7` forms of the SSE2 packed integer instructions (`padd*`, `psub*`, `pcmp*`, `pack*`, `punpck*`, shifts, logic)

</details>

<details>
<summary>BMI1 (0/6)</summary>

- [ ] `andn`
- [ ] `bextr`
- [ ] `blsi`
- [ ] `blsmsk`
- [ ] `blsr`
- [ ] `tzcnt`

</details>

<details>
<summary>BMI2 (0/8)</summary>

- [ ] `bzhi`
- [ ] `mulx`
- [ ] `pdep`
- [ ] `pext`
- [ ] `rorx`
- [ ] `sarx`
- [ ] `shlx`
- [ ] `shrx`

</details>

<details>
<summary>POPCNT and LZCNT (0/2)</summary>

- [ ] `popcnt`
- [ ] `lzcnt`

</details>

<details>
<summary>AES-NI and PCLMULQDQ (0/7)</summary>

- [ ] `aesenc`
- [ ] `aesenclast`
- [ ] `aesdec`
- [ ] `aesdeclast`
- [ ] `aesimc`
- [ ] `aeskeygenassist`
- [ ] `pclmulqdq`

</details>

<details>
<summary>SHA (0/7)</summary>

- [ ] `sha1rnds4`
- [ ] `sha1nexte`
- [ ] `sha1msg1`
- [ ] `sha1msg2`
- [ ] `sha256rnds2`
- [ ] `sha256msg1`
- [ ] `sha256msg2`

</details>

<details>
<summary>XSAVE (0/8)</summary>

- [ ] `xsave`
- [ ] `xrstor`
- [ ] `xsaveopt`
- [ ] `xsavec`
- [ ] `xsaves`
- [ ] `xrstors`
- [ ] `xgetbv`
- [ ] `xsetbv`

</details>

<details>
<summary>CET (0/10)</summary>

`endbr64` shows up at the start of most functions in code compiled with `-fcf-protection`. It runs as a `nop` when CET is off, so it's worth doing early.

- [ ] `endbr64`
- [ ] `endbr32`
- [ ] `incsspq`
- [ ] `rdsspq`
- [ ] `saveprevssp`
- [ ] `rstorssp`
- [ ] `wrssq`
- [ ] `wrussq`
- [ ] `setssbsy`
- [ ] `clrssbsy`

</details>

<details>
<summary>VMX: Intel VT-x (0/13)</summary>

- [ ] `vmxon`
- [ ] `vmxoff`
- [ ] `vmclear`
- [ ] `vmptrld`
- [ ] `vmptrst`
- [ ] `vmread`
- [ ] `vmwrite`
- [ ] `vmlaunch`
- [ ] `vmresume`
- [ ] `vmcall`
- [ ] `vmfunc`
- [ ] `invept`
- [ ] `invvpid`

</details>

<details>
<summary>SVM: AMD-V (0/8)</summary>

- [ ] `vmrun`
- [ ] `vmload`
- [ ] `vmsave`
- [ ] `vmmcall`
- [ ] `stgi`
- [ ] `clgi`
- [ ] `skinit`
- [ ] `invlpga`

</details>

<details>
<summary>Other small extensions (0/17)</summary>

- [ ] `movbe` (MOVBE)
- [ ] `rdrand` (RDRAND)
- [ ] `rdseed` (RDSEED)
- [ ] `adcx` (ADX)
- [ ] `adox` (ADX)
- [ ] `rdfsbase` (FSGSBASE)
- [ ] `rdgsbase` (FSGSBASE)
- [ ] `wrfsbase` (FSGSBASE)
- [ ] `wrgsbase` (FSGSBASE)
- [ ] `clflushopt` (CLFLUSHOPT)
- [ ] `clwb` (CLWB)
- [ ] `rdpid` (RDPID)
- [ ] `prefetchw` (PREFETCHW)
- [ ] `xbegin` (TSX)
- [ ] `xend` (TSX)
- [ ] `xabort` (TSX)
- [ ] `xtest` (TSX)

</details>
