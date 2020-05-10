// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Holds the registers and register-like values that are callee-saved.
///
/// Occupies 64 bytes.
#[repr(C, align(16))]
struct SavedContext
{
	/// Value is set to default of `0x00001F80` when first initialized.
	///
	/// This is because:-
	///
	/// * Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 2A, Section "`LDMXCSR`—Load `MXCSR` Register": "The default `MXCSR` value at reset is `1F80H`".
	/// * Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, Section 10.2.3 "MXCSR Control and Status Register": "Bits 16 through 31 of the `MXCSR` register are reserved ... attempting to write a non-zero value to these bits, using either the `FXRSTOR` or `LDMXCSR` instructions, will result in a general-protection exception (`#GP`) being generated".
	sse_control_and_status_word: u32,

	/// Value is set to default of `0x037F`. when first initialized.
	///
	/// This is because:-
	///
	/// * Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, Section 8.1.5 "x87 FPU Control Word": "When the x87 FPU is initialized ... the x87 FPU control word is set to `037FH`".
	x87_control_word: u16,

	_padding: u16,

	/// Contains `context_entry_function_pointer` when first initialized.
	///
	/// Otherwise value is saved value of register RBX.
	register_rbx_value: u64,

	/// Contains the absolute address of the label `terminate_abnormally` when first initialized.
	///
	/// Otherwise value is saved value of register RBP.
	register_rbp_value: u64,

	/// Value is uninitialized when first initialized.
	///
	/// Otherwise value is saved value of register R12.
	register_r12_value: u64,

	/// Value is uninitialized when first initialized.
	///
	/// Otherwise value is saved value of register R13.
	register_r13_value: u64,

	/// Value is uninitialized when first initialized.
	///
	/// Otherwise value is saved value of register R14.
	register_r14_value: u64,

	/// Value is uninitialized when first initialized.
	///
	/// Otherwise value is saved value of register R15.
	register_r15_value: u64,

	/// Contains the absolute address of the label `trampoline` when first initialized.
	///
	/// Subsequently contains a location to jump to resume execution, ie the absolute value of the `RIP` register.
	///
	/// This field ***MUST*** be the last field, as it needs to overlap (in a sort of mis-placed union) with the return address of a function on the stack.
	resume_instruction_pointer: u64,
}

// `trampoline`.
//
// Used on first call to `resume()` as the resume of execution point.
//
// Use a sequence of `push, jmp` which is identical to a `CALL` instruction.
// After restoring the initial context in `resume()`, rbp will contain `context_entry_function_pointer` and rbx will contain `terminate_abnormally`.
global_asm!
{r#"
    .intel_syntax noprefix
	.text
	.p2align 4, 0x90
	trampoline:

		// After initial restore the address of the `terminate_abnormally` address is in rbp.
		// We push this as-if it were the return address of a `CALL` instruction.
		// If the logic called by `jmp rbx` ever returns using `ret`\*, it will then pop rbp from the stack as the return address, and so execute the code at `terminate_abnormally`.
		// \* It should not as it returns `-> !`; see the comments below for `terminate_abnormally`.
		push rbp

		// After intial restore `context_entry_function_pointer` is in rbx.
		jmp rbx
"#}

// `terminate_abnormally`.
//
// If the context function `context_entry_function_pointer: ContextEntryPointFunctionPointer` ever returns (it should not; in Rust it is defined as returning `-> !`), then this code will execute.
//
// It raises an invalid opcode exception (which is identical to what `std::intrinsics::unreachable()` would do in Rust).
global_asm!
{r#"
    .intel_syntax noprefix
	.text
	.p2align 4, 0x90
	terminate_abnormally:

		// Raises an invalid opcode exception (`#UD`).
		ud2
"#}

impl SavedContext
{
	/// Must never be inlined, as the design takes advantage of the System V ABI calling convention for x86-64 to preserve registers.
	///
	/// If called during an Intel hardware memory transaction (see Intel TSX / HTM), will cause the transaction to be aborted everytime.
	///
	/// In this calling convention, the majority of registers and extended state is caller-saved.
	/// Hence only a small amount of state needs to be preserved.
	///
	/// See the documentation of the `Stack` trait as to why `pointer_to_bottom_of_stack` is the ***highest*** address: on x86-64, stacks grow downwards.
	#[inline(never)]
	#[no_mangle]
	#[naked]
	#[allow(unused_variables)]
	unsafe extern "C" fn initialize(pointer_to_bottom_of_stack: *const u8, context_entry_function_pointer: ContextEntryPointFunctionPointer) -> NonNull<SavedContext>
	{
		asm!
		(
		"
			// (1) Save initial context by partially initializing `SavedContext`.

				// Reserve 64 bytes below `pointer_to_bottom_of_stack` (rdi) for `size_of::<SavedContext>()`.
				lea rdi, qword ptr [rdi - 64]

				// Save a known good initial state that can be restored into the MXCSR register (ie does not raise an exception on load) in the `SavedContext.sse_control_and_status_word` field.
				// Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 2A, Section \"LDMXCSR—Load MXCSR Register\": \"The default MXCSR value at reset is 1F80H\".
				// Using an immediate with MOV is an optimization over using the instruction `stmxcsr [rdi]`.
				mov dword ptr [rdi], dword ptr 0x00001F80

				// Save a known good initial state that can be restored into the FPU x87 control word (ie does not raise an exception on load) in the `SavedContext.x87_control_word` field.
				// Intel® 64 and IA-32 Architectures Software Developer’s Manual, Volume 1, Section 8.1.5 \"x87 FPU Control Word\": \"When the x87 FPU is initialized ... the x87 FPU control word is set to 037FH\".
				// Using an immediate with MOV is an optimization over using the instruction `fnstcw [rdi + 4]`.
				mov word ptr [rdi + 4], word ptr 0x037F

				// Store second (one-based) argument `context_entry_function_pointer` (rsi) in the `SavedContext.register_rbx_value` field.
				mov qword ptr [rdi + 8], rsi

				// Store the address of the label `terminate_abnormally` (rax) in the `SavedContext.register_rbp_value` field.
				lea rax, qword ptr terminate_abnormally[rip]
				mov qword ptr [rdi + 16], rax

				// Callee-saved registers r12 - r15 can be restored to any value and so do not need to be recorded.

				// Store the address of the label `trampoline` (rax) in the `SavedContext.resume_instruction_pointer` field.
				// This ensures that on the very first call to the context, after it has been restored in `resume()`, it will call the 'function' `trampoline`.
				lea rax, qword ptr trampoline[rip]
				mov qword ptr [rdi + 56], rax


			// (2) Returns `pointer_to_bottom_of_stack - 64` (rax); this is a pointer to the initial `SavedContext`.
			mov rax, rdi
			ret
		"
		:
			// Output constraints.
		:
			// Input constraints.
		:
			// Clobbers.
			"~rax",
			"~rcx"
		:
			// Options.
			"volatile",
			"intel"
		);
		unreachable()
	}

	/// Must never be inlined, as the design takes advantage of the System V ABI calling convention for x86-64.
	///
	/// If called during an Intel hardware memory transaction (see Intel TSX / HTM), will cause the transaction to be aborted everytime.
	#[inline(never)]
	#[no_mangle]
	#[naked]
	#[allow(unused_variables)]
	unsafe extern "C" fn resume(pointer_to_previously_saved_stack_context: NonNull<SavedContext>, data_to_transfer: DataToTransfer) -> Transfer
	{
		asm!
		(
		"
			// (1) Save the current context's register and associated state onto the stack (rsp) into `pointer_to_newly_saved_stack_context` (of type `NonNull<SavedContext>`).

				// Reserve 56 bytes of stack space (not 64, the `size_of::<SavedContext>()`).
				// This is because the 8 bytes above the current stack pointer (rsp) contain the return address, making 64 (the `CALL` instruction that called us (sic) will have pushed the 8 byte return address onto the stack (ie it pushed rip)).
				// Hence the `SavedContext.resume_instruction_pointer` field is already populated (saved).
				// Save `pointer_to_newly_saved_stack_context` (rsp - 56) in rax.
				lea rax, [rsp - 56]

				// Save MMX control word and status word in the `SavedContext.sse_control_and_status_word` field.
				stmxcsr [rax]

				// Save x87 control word in the `SavedContext.x87_control_word` field.
				fnstcw [rax + 4]

				// Save callee-saved registers (rbx, rbp and r12 - r15).

					// Save rbx in the `SavedContext.register_rbx_value` field.
					mov qword ptr [rax + 8], rbx

					// Save rbp in the `SavedContext.register_rbp_value` field.
					mov qword ptr [rax + 16], rbp

					// Save r12 in the `SavedContext.register_r12_value` field.
					mov qword ptr [rax + 24], r12

					// Save r13 in the `SavedContext.register_r13_value` field.
					mov qword ptr [rax + 32], r13

					// Save r14 in the `SavedContext.register_r14_value` field.
					mov qword ptr [rax + 30], r14

					// Save r15 in the `SavedContext.register_r15_value` field.
					mov qword ptr [rax + 48], r15

					// rip is already saved in the `SavedContext.register_rbp_value` field as it is 8 bytes above the original stack pointer (rax + 56 or rsp).


			// (2) Restore previous context register and associated state from the first (one-based) argument passed `pointer_to_previously_saved_stack_context` (rdi).

				// Restore MMX control word and status word from the `pointer_to_previously_saved_stack_context.sse_control_and_status_word` field.
				ldmxcsr [rdi]

				// Restore x87 control word from the `pointer_to_previously_saved_stack_context.x87_control_word` field.
				fldcw [rdi + 4]

				// Restore callee-saved registers (rbx, rbp and r12 - r15).

					// Restore rbx from the `pointer_to_previously_saved_stack_context.register_rbx_value` field.
					// After intial restore `context_entry_function_pointer` is in the `pointer_to_previously_saved_stack_context.register_rbx_value` field.
					mov rbx, qword ptr [rdi + 8]

					// Restore rbp from the `SavedContext.register_rbp_value` field.
					// After initial restore the address of the `terminate_abnormally` label is in the `pointer_to_previously_saved_stack_context.register_rbp_value`.
					mov rbp, qword ptr [rdi + 16]

					// Restore r12 from the `pointer_to_previously_saved_stack_context.register_r12_value` field.
					mov r12, qword ptr [rdi + 24]

					// Restore r13 from the `pointer_to_previously_saved_stack_context.register_r13_value` field.
					mov r13, qword ptr [rdi + 32]

					// Restore r14 from the `pointer_to_previously_saved_stack_context.register_r14_value` field.
					mov r14, qword ptr [rdi + 40]

					// Restore r15 from the `pointer_to_previously_saved_stack_context.register_r15_value` field.
					mov r15, qword ptr [rdi + 48]

				// Restore return address (into r8) from the `pointer_to_previously_saved_stack_context.resume_instruction_pointer` field.
				// After initial restore the address of the `trampoline` label is in the `pointer_to_previously_saved_stack_context.resume_instruction_pointer` field.
				mov r8, qword ptr [rdi + 56]


			// (3) Re-enter previous context function.

				// Point stack to highest address of `pointer_to_previously_saved_stack_context` (rdi).
				//
				// The data below `pointer_to_previously_saved_stack_context` is going to be overwritten when the resumed context function is called.
				lea rsp, qword ptr [rdi + 64]

				// We return `Transfer`, which is a tuple pair `(NonNull<SavedContext>, DataToTransfer)`.
				// In the x86-64 System V ABI, a tuple pair can be returned in registers: `rax:rdx`.

					// Move `pointer_to_newly_saved_stack_context` (rax) into the return field `Transfer.data_passed_from_previously_executed_context` (rax).
					// (nothing to do).

					// Move `data_to_transfer` (rsi) into the return field `Transfer.data_passed_from_previously_executed_context` (rdx).
					mov rdx, rsi

				// Pass `Transfer` to the first argument, spanning two registers, of the context function `context_entry_function_pointer: ContextEntryPointFunctionPointer`.
				// In the x86-64 System V ABI, a tuple pair for the first argument can span two registers: `rdi:rsi`.

					// Move `pointer_to_newly_saved_stack_context` (rax) to `Transfer.previously_executed_context_which_yielded_to_resume_the_current_context` (rdi).
					mov rdi, rax

					// Move `data_to_transfer` (rsi) to `Transfer.data_passed_from_previously_executed_context` (rsi).
					// (nothing to do).

				// Indirect jump to either:-
				// * Enter the context function `context_entry_function_pointer: ContextEntryPointFunctionPointer` once after initial restore, or,
				// * Resume context function; when this finishes it will return to just below the `jmp`, and, because we were `CALL`d, pop the return address off the stack.
				jmp r8
		"
		:
			// Output constraints.
		:
			// Input constraints.
		:
			// Clobbers.
			"~rsp"
			"~rax"
			"~r8"
			"~rdi"
			"~rdx"
		:
			// Options.
			"volatile",
			"intel"
		);
		unreachable()
	}
}
