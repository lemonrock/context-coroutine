// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Holds a pointer to the registers and register-like values that are callee-saved.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct SavedContextWrapper(NonNull<SavedContext>);

#[cfg(all(unix, target_arch = "x86_64"))]
impl SavedContextWrapper
{
	/// Creates a new `SavedContextWrapper` prepared to execute `context_entry_point_function_pointer` at the beginning of `stack`.
	///
	/// `context_entry_point_function_pointer` is not executed until the first call to `resume()`.
	///
	/// It is your responsibility to make sure `stack` lives longer than the generated `Self` result.
	#[inline(always)]
	fn initialize(stack: &impl Stack, context_entry_point_function_pointer: ContextEntryPointFunctionPointer) -> Self
	{
		Self(unsafe { SavedContext::initialize(stack.bottom(), context_entry_point_function_pointer) })
	}

	/// Yields execution to this `SavedContextWrapper` (`self`).
	///
	/// The current state of execution is preserved somewhere (eg within memory reserved in the stack) and the previously saved state in the `Context` pointed to by `self` is restored and executed next.
	///
	/// This behaviour is similiar in spirit to regular function calls with the difference that the call to `resume()` only returns when someone resumes the caller in turn.
	///
	/// If called on the generated `SavedContextWrapper` result of `initialize()` then starts execution at the beginning of the `context_entry_point_function_pointer` passed to `new()`.
	///
	/// The restored and executed context will be passed a `Transfer` object whose's `data_passed_from_previously_executed_context` will be `data_to_transfer` and whose `previously_executed_context_which_yielded_to_resume_the_current_context` will be the current state of execution before preservation.
	///
	/// The returned `Transfer` struct contains the previously active `SavedContextWrapper` (`Transfer.previously_executed_context_which_yielded_to_resume_the_current_context`) and the `data_to_transfer` argument used to resume the current one (`Transfer.data_passed_from_previously_executed_context`).
	///
	/// It is your responsibility to make sure that all `data_to_transfer` that constructed in this context has been dropped properly when the last context is dropped.
	#[inline(always)]
	fn resume(self, data_to_transfer: DataToTransfer) -> Transfer
	{
		unsafe { SavedContext::resume(self.0, data_to_transfer) }
	}
}
