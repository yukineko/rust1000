	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.private_extern	__ZN3std2rt10lang_start17hc9ec6b6c1c21b466E
	.globl	__ZN3std2rt10lang_start17hc9ec6b6c1c21b466E
	.p2align	2
__ZN3std2rt10lang_start17hc9ec6b6c1c21b466E:
	.cfi_startproc
	sub	sp, sp, #48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	str	x1, [sp, #8]
	mov	x0, x2
	ldr	x2, [sp, #8]
	str	x0, [sp, #16]
	mov	x4, x3
	ldr	x3, [sp, #16]
	sub	x0, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_1@PAGE
	add	x1, x1, l___unnamed_1@PAGEOFF
	bl	__ZN3std2rt19lang_start_internal17h95cf27b851151b9cE
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	ret
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfd1fe55ffac695a8E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std3sys9backtrace28__rust_begin_short_backtrace17h44e3fcc15ba029bbE
	bl	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcc3ea3c6bcbd09d6E
	and	w0, w0, #0xff
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.p2align	2
__ZN3std3sys9backtrace28__rust_begin_short_backtrace17h44e3fcc15ba029bbE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4core3ops8function6FnOnce9call_once17h7d3f893992e17b9aE
	; InlineAsm Start
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments9new_const17h762cab0d31c0a37fE:
	.cfi_startproc
	str	x0, [x8]
	mov	w9, #1
	str	x9, [x8, #8]
	adrp	x9, l___unnamed_2@PAGE
	adrp	x10, l___unnamed_2@PAGE
	add	x10, x10, l___unnamed_2@PAGEOFF
	ldr	x11, [x9, l___unnamed_2@PAGEOFF]
	mov	w9, #8
	ldr	x10, [x10, #8]
	str	x11, [x8, #32]
	str	x10, [x8, #40]
	str	x9, [x8, #16]
	str	xzr, [x8, #24]
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf0e802d55956103eE:
	.cfi_startproc
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3ops8function6FnOnce9call_once17h83e29275c978ee6eE
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h7d3f893992e17b9aE:
	.cfi_startproc
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h83e29275c978ee6eE:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	add	x0, sp, #16
	str	x8, [sp, #16]
Ltmp0:
	bl	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfd1fe55ffac695a8E
	str	w0, [sp, #12]
Ltmp1:
	b	LBB6_3
LBB6_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB6_2:
Ltmp2:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB6_1
LBB6_3:
	ldr	w0, [sp, #12]
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	ret
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table6:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h78975a253d878bd5E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hcc3ea3c6bcbd09d6E:
	.cfi_startproc
	mov	w0, #0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main4main17h9b68587896d3bd06E:
	.cfi_startproc
	sub	sp, sp, #80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	add	x8, sp, #16
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_3@PAGE
	add	x0, x0, l___unnamed_3@PAGEOFF
	bl	__ZN4core3fmt9Arguments9new_const17h762cab0d31c0a37fE
	ldr	x0, [sp, #8]
	bl	__ZN3std2io5stdio6_print17h6200d46cef53dee1E
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x1
	mov	x8, x0
	sxtw	x1, w8
	adrp	x0, __ZN4main4main17h9b68587896d3bd06E@PAGE
	add	x0, x0, __ZN4main4main17h9b68587896d3bd06E@PAGEOFF
	mov	w3, #0
	bl	__ZN3std2rt10lang_start17hc9ec6b6c1c21b466E
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hf0e802d55956103eE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfd1fe55ffac695a8E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfd1fe55ffac695a8E

	.section	__TEXT,__literal16,16byte_literals
	.p2align	3, 0x0
l___unnamed_2:
	.space	8
	.space	8

	.section	__TEXT,__const
l___unnamed_4:
	.ascii	"Hello, world!\n"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_3:
	.quad	l___unnamed_4
	.asciz	"\016\000\000\000\000\000\000"

.subsections_via_symbols
