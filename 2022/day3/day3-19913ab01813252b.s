	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.p2align	2
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h74351e2aa1bd1922E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	; InlineAsm Start
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17h7a7f78447145eb2aE
	.globl	__ZN3std2rt10lang_start17h7a7f78447145eb2aE
	.p2align	2
__ZN3std2rt10lang_start17h7a7f78447145eb2aE:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x4, x3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
Lloh0:
	adrp	x1, l___unnamed_1@PAGE
Lloh1:
	add	x1, x1, l___unnamed_1@PAGEOFF
	add	x0, sp, #8
	bl	__ZN3std2rt19lang_start_internal17h18883ab0303ff1c2E
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh0, Lloh1

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h6b7a0f11e8c2cbb2E:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h74351e2aa1bd1922E
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h15383e4a8947f557E:
	stp	x20, x19, [sp, #-32]!
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x19, x1
	ldr	x20, [x0]
	mov	x0, x1
	bl	__ZN4core3fmt9Formatter15debug_lower_hex17h76716a2a5c71d388E
	tbz	w0, #0, LBB3_2
	mov	x0, x20
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	__ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u32$GT$3fmt17hf64c770bd5ad40ffE
LBB3_2:
	mov	x0, x19
	bl	__ZN4core3fmt9Formatter15debug_upper_hex17h61022d0980057811E
	tbz	w0, #0, LBB3_4
	mov	x0, x20
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	__ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u32$GT$3fmt17h0507b85aacde413aE
LBB3_4:
	mov	x0, x20
	mov	x1, x19
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h5ce3300cf5b51408E

	.p2align	2
__ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdbb5e148b856e9cdE:
	mov	x2, x1
	ldp	x8, x1, [x0]
	mov	x0, x8
	b	__ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hcfd1f9d276970b8eE

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h47199f70f434feabE:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h74351e2aa1bd1922E
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h0f096961496bb8a9E:
	.cfi_startproc
	stp	x20, x19, [sp, #-32]!
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	ldr	x19, [x0]
	and	x8, x19, #0x3
	cmp	x8, #1
	b.ne	LBB6_4
	ldr	x0, [x19, #-1]!
	ldr	x8, [x19, #8]
	ldr	x8, [x8]
	blr	x8
	ldr	x8, [x19, #8]
	ldr	x1, [x8, #8]
	cbz	x1, LBB6_3
	ldr	x2, [x8, #16]
	ldr	x0, [x19]
	bl	___rust_dealloc
LBB6_3:
	mov	x0, x19
	mov	w1, #24
	mov	w2, #8
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	b	___rust_dealloc
LBB6_4:
	ldp	x29, x30, [sp, #16]
	ldp	x20, x19, [sp], #32
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hba06408469aa128cE:
	ret

	.p2align	2
__ZN4day34main17hdd733402e6f71d5eE:
	.cfi_startproc
	sub	sp, sp, #160
	.cfi_def_cfa_offset 160
	stp	x20, x19, [sp, #128]
	stp	x29, x30, [sp, #144]
	add	x29, sp, #144
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
Lloh2:
	adrp	x0, l___unnamed_2@PAGE
Lloh3:
	add	x0, x0, l___unnamed_2@PAGEOFF
	add	x8, sp, #56
	mov	w1, #11
	bl	__ZN3std2fs4read5inner17hc4c401655ff0c010E
	ldr	x0, [sp, #64]
	cbz	x0, LBB8_28
	mov	w19, #0
	ldr	x1, [sp, #56]
	ldr	x8, [sp, #72]
	mov	w9, #1
	mov	x10, x0
	b	LBB8_3
LBB8_2:
	add	x10, x13, x14
	add	x10, x10, #1
	mvn	x12, x14
	add	x8, x12, x8
	cbnz	x14, LBB8_8
LBB8_3:
	cbz	x8, LBB8_24
	mov	x13, x10
	mov	x14, #0
LBB8_5:
	ldrb	w11, [x13, x14]
	cmp	w11, #10
	b.eq	LBB8_2
	add	x14, x14, #1
	cmp	x8, x14
	b.ne	LBB8_5
	mov	x14, x8
	mov	x10, x13
LBB8_8:
	lsr	x16, x14, #1
	cmp	x14, #1
	b.ne	LBB8_10
	mov	x12, #0
	b	LBB8_15
LBB8_10:
	mov	x12, #0
	mov	x15, x16
	mov	x17, x13
	b	LBB8_12
LBB8_11:
	mov	w3, #160
	add	w2, w3, w2
	lsl	x2, x9, x2
	orr	x12, x2, x12
	subs	x15, x15, #1
	b.eq	LBB8_15
LBB8_12:
	ldrb	w2, [x17], #1
	sub	w3, w2, #97
	cmp	w3, #26
	b.lo	LBB8_11
	sub	w3, w2, #65
	cmp	w3, #26
	b.hs	LBB8_27
	mov	w3, #218
	add	w2, w3, w2
	lsl	x2, x9, x2
	orr	x12, x2, x12
	subs	x15, x15, #1
	b.ne	LBB8_12
LBB8_15:
	mov	x15, #0
	mov	x17, #0
	add	x13, x13, x16
	sub	x14, x14, x16
	b	LBB8_18
LBB8_16:
	mov	w2, #160
LBB8_17:
	add	w16, w2, w16
	lsl	x16, x9, x16
	orr	x2, x16, x17
	tst	x16, x12
	csel	x15, x15, x2, eq
	csel	x17, x17, x2, eq
	subs	x14, x14, #1
	b.eq	LBB8_21
LBB8_18:
	ldrb	w16, [x13], #1
	sub	w2, w16, #97
	cmp	w2, #26
	b.lo	LBB8_16
	sub	w2, w16, #65
	cmp	w2, #26
	b.hs	LBB8_27
	mov	w2, #218
	b	LBB8_17
LBB8_21:
	mov	w12, #0
	cbz	x15, LBB8_23
LBB8_22:
	rbit	x13, x15
	clz	x13, x13
	lsl	x14, x9, x13
	add	w12, w12, w13
	bics	x15, x15, x14
	b.ne	LBB8_22
LBB8_23:
	add	w19, w12, w19
	cmp	w11, #10
	b.eq	LBB8_3
LBB8_24:
	cbz	x1, LBB8_26
	mvn	x8, x1
	lsr	x2, x8, #63
	bl	___rust_dealloc
LBB8_26:
	str	w19, [sp, #4]
	add	x8, sp, #4
	stur	x8, [x29, #-24]
Lloh4:
	adrp	x8, l___unnamed_3@PAGE
Lloh5:
	add	x8, x8, l___unnamed_3@PAGEOFF
Lloh6:
	adrp	x9, __ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdbb5e148b856e9cdE@PAGE
Lloh7:
	add	x9, x9, __ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hdbb5e148b856e9cdE@PAGEOFF
	stp	x8, x9, [sp, #56]
Lloh8:
	adrp	x8, l___unnamed_4@PAGE
Lloh9:
	add	x8, x8, l___unnamed_4@PAGEOFF
Lloh10:
	adrp	x10, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h5ce3300cf5b51408E@GOTPAGE
Lloh11:
	ldr	x10, [x10, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h5ce3300cf5b51408E@GOTPAGEOFF]
	stp	x8, x10, [sp, #72]
Lloh12:
	adrp	x8, l___unnamed_5@PAGE
Lloh13:
	add	x8, x8, l___unnamed_5@PAGEOFF
	stp	x8, x9, [sp, #88]
	sub	x8, x29, #24
Lloh14:
	adrp	x9, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h15383e4a8947f557E@PAGE
Lloh15:
	add	x9, x9, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h15383e4a8947f557E@PAGEOFF
Lloh16:
	adrp	x10, l___unnamed_6@PAGE
Lloh17:
	add	x10, x10, l___unnamed_6@PAGEOFF
	stp	x8, x9, [sp, #104]
	mov	w8, #5
	stp	x10, x8, [sp, #24]
Lloh18:
	adrp	x8, l___unnamed_7@PAGE
Lloh19:
	add	x8, x8, l___unnamed_7@PAGEOFF
	mov	w9, #4
	stp	x8, x9, [sp, #8]
	add	x8, sp, #56
	stp	x8, x9, [sp, #40]
	add	x0, sp, #8
	bl	__ZN3std2io5stdio7_eprint17hf03856f0e53ae7d2E
	ldp	x29, x30, [sp, #144]
	ldp	x20, x19, [sp, #128]
	add	sp, sp, #160
	ret
LBB8_27:
Lloh20:
	adrp	x8, l___unnamed_8@PAGE
Lloh21:
	add	x8, x8, l___unnamed_8@PAGEOFF
	mov	w9, #1
	stp	x8, x9, [sp, #72]
	str	xzr, [sp, #56]
Lloh22:
	adrp	x8, l___unnamed_9@PAGE
Lloh23:
	add	x8, x8, l___unnamed_9@PAGEOFF
	stp	x8, xzr, [sp, #88]
Lloh24:
	adrp	x1, l___unnamed_10@PAGE
Lloh25:
	add	x1, x1, l___unnamed_10@PAGEOFF
	add	x0, sp, #56
	bl	__ZN4core9panicking9panic_fmt17h179ef32acccc5654E
LBB8_28:
	ldr	x8, [sp, #56]
	str	x8, [sp, #8]
Lloh26:
	adrp	x0, l___unnamed_11@PAGE
Lloh27:
	add	x0, x0, l___unnamed_11@PAGEOFF
Lloh28:
	adrp	x3, l___unnamed_12@PAGE
Lloh29:
	add	x3, x3, l___unnamed_12@PAGEOFF
Lloh30:
	adrp	x4, l___unnamed_13@PAGE
Lloh31:
	add	x4, x4, l___unnamed_13@PAGEOFF
	add	x2, sp, #8
	mov	w1, #43
	bl	__ZN4core6result13unwrap_failed17h709c541c76cd2d89E
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpLdrGot	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x3, x1
	sxtw	x2, w0
Lloh32:
	adrp	x8, __ZN4day34main17hdd733402e6f71d5eE@PAGE
Lloh33:
	add	x8, x8, __ZN4day34main17hdd733402e6f71d5eE@PAGEOFF
	str	x8, [sp, #8]
Lloh34:
	adrp	x1, l___unnamed_1@PAGE
Lloh35:
	add	x1, x1, l___unnamed_1@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__ZN3std2rt19lang_start_internal17h18883ab0303ff1c2E
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33

	.section	__DATA,__const
	.p2align	3
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hba06408469aa128cE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h47199f70f434feabE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h6b7a0f11e8c2cbb2E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h6b7a0f11e8c2cbb2E

	.section	__TEXT,__const
	.p2align	3
l___unnamed_9:
	.byte	0

l___unnamed_11:
	.ascii	"called `Result::unwrap()` on an `Err` value"

	.section	__DATA,__const
	.p2align	3
l___unnamed_12:
	.quad	__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h0f096961496bb8a9E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h58ebc9d9fee9edc2E

	.section	__TEXT,__const
l___unnamed_14:
	.ascii	"Unknown item"

	.section	__DATA,__const
	.p2align	3
l___unnamed_8:
	.quad	l___unnamed_14
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_15:
	.ascii	"src/main.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_10:
	.quad	l___unnamed_15
	.asciz	"\013\000\000\000\000\000\000\000\020\000\000\000\t\000\000"

	.section	__TEXT,__const
l___unnamed_2:
	.ascii	"./input.txt"

	.section	__DATA,__const
	.p2align	3
l___unnamed_13:
	.quad	l___unnamed_15
	.asciz	"\013\000\000\000\000\000\000\000r\000\000\000\n\000\000"

	.section	__TEXT,__const
l___unnamed_16:
	.byte	91

l___unnamed_17:
	.byte	58

l___unnamed_18:
	.ascii	"] "

l___unnamed_19:
	.ascii	" = "

l___unnamed_20:
	.byte	10

	.section	__DATA,__const
	.p2align	3
l___unnamed_6:
	.quad	l___unnamed_16
	.asciz	"\001\000\000\000\000\000\000"
	.quad	l___unnamed_17
	.asciz	"\001\000\000\000\000\000\000"
	.quad	l___unnamed_18
	.asciz	"\002\000\000\000\000\000\000"
	.quad	l___unnamed_19
	.asciz	"\003\000\000\000\000\000\000"
	.quad	l___unnamed_20
	.asciz	"\001\000\000\000\000\000\000"

	.p2align	3
l___unnamed_3:
	.quad	l___unnamed_15
	.asciz	"\013\000\000\000\000\000\000"

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2
l___unnamed_4:
	.asciz	"\214\000\000"

	.section	__TEXT,__const
l___unnamed_21:
	.ascii	"result"

	.section	__DATA,__const
	.p2align	3
l___unnamed_5:
	.quad	l___unnamed_21
	.asciz	"\006\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3
l___unnamed_7:
	.asciz	"\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\003\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\003\000\000\000\000\000\000\000\001\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\003\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\004\000\000\000 \000\000\000\003\000\000\000\000\000\000\000\003\000\000\000\000\000\000"

.subsections_via_symbols
