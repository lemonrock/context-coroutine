// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// Information to transfer between contexts.
///
/// Transferring between contexts switches the stack in use.
///
/// Use `TypeSafeTransfer` in preference to this low-level construction.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Transfer
{
	/// The previously executed `SavedContextWrapper` which yielded to resume the current one.
	previously_executed_context_which_yielded_to_resume_the_current_context: SavedContextWrapper,

	/// The `data` which was passed to `Context::resume()` to resume the current `SavedContextWrapper`.
	///
	/// The initial value of this is meaningless.
	data_passed_from_previously_executed_context: DataToTransfer,
}

impl Transfer
{
	/// Creates a new instance, by initializing a new context.
	///
	/// It is your responsibility to make sure `stack` lives longer than the generated `Self` result.
	#[inline(always)]
	pub fn new(stack: &impl Stack, context_entry_point_function_pointer: ContextEntryPointFunctionPointer) -> Self
	{
		Self
		{
			previously_executed_context_which_yielded_to_resume_the_current_context: SavedContextWrapper::initialize(stack, context_entry_point_function_pointer),
			data_passed_from_previously_executed_context: unsafe_uninitialized(),
		}
	}

	/// Resume.
	///
	/// It is your responsibility to make sure `data_to_transfer` lives longer than the stack above this function call; use `TypeSafeTransfer` in preference to this code.
	#[inline(always)]
	pub fn resume<TD: TransferableData>(&mut self, data_to_transfer: TD)
	{
		*self = self.previously_executed_context_which_yielded_to_resume_the_current_context.resume(data_to_transfer.into_usize())
	}

	/// Get data.
	///
	/// It is your responsibility to make sure `data_to_transfer` lives longer than the stack above this function call; use `TypeSafeTransfer` in preference to this code.
	#[inline(always)]
	pub fn transferred_data<TD: TransferableData>(&self) -> TD
	{
		TD::from_usize(self.data_passed_from_previously_executed_context)
	}
}
