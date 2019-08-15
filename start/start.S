	.align 4
	.section ".text.start"
	.global _start

_start:
	// disable all interrupt
	msr DAIFSet, #0x0F

	// initialize stack
	ldr x0, =__stack_start
	mov sp, x0

	// clear bss
	bl bss_clear

	// set EL2 interrupt vector table
	bl set_vector_table

	// start main program
	bl main

	// do nothing
	b finish

	.section ".text"
bss_clear:
	ldr x1, =__bss_start
	ldr x2, =__bss_end
1:	// clear loop
	cmp x1, x2
	beq 2f			// break
	str wzr, [x1]
	add x1, x1, #4
	b 1b			// clear loop end
2:
	ret

set_vector_table:
	ldr x1, =vector_table_el2
	msr VBAR_EL2, x1
	ret
	
finish:
	// halt
	wfe
	b finish
	
