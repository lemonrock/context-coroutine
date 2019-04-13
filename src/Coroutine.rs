// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// A trait that stackful coroutines should implement.
///
/// Start a new instance of this coroutine by using `Self::start_coroutine()`.
pub trait Coroutine: Sized
{
	/// Type of the arguments the coroutine is initially called with, eg `(usize, String)`.
	type StartArguments: Sized;

	/// Type of the arguments the coroutine is resumed with, eg `(u8, Vec<f64>)`.
	type ResumeArguments: Sized;

	/// Type of the result from a yield of the coroutine.
	type Yields: Sized;

	/// Type of the final result from the coroutine.
	type Complete: Sized;

	/// Implement this for the coroutine's behaviour.
	///
	/// Panics inside the coroutine are transferred to the calling thread and raised.
	fn coroutine<'yielder>(start_arguments: Self::StartArguments, yielder: Yielder<'yielder, Self::ResumeArguments, Self::Yields, Self::Complete>) -> Self::Complete;

	/// Starts the coroutine; execution will transfer to the coroutine.
	///
	/// Execution does not start (returns `Err(AllocErr)`) if there is not memory available to start the coroutine.
	///
	/// Ownership of `start_arguments` will also transfer.
	///
	/// Returns the data transferred to us after the start and a guard object to resume the coroutine again or the final result.
	///
	/// If the coroutine panicked, this panics.
	#[inline(always)]
	fn start_coroutine<GTACSA: GlobalThreadAndCoroutineSwitchableAllocator>(coroutine_memory_source: &CoroutineMemorySource<GTACSA>, start_arguments: Self::StartArguments) -> Result<StartOutcome<GTACSA, Self>, AllocErr>
	{
		let coroutine_memory = coroutine_memory_source.allocate_coroutine_memory()?;

		Ok(CoroutineInstance::new(coroutine_memory).start(start_arguments))
	}

	#[doc(hidden)]
	#[inline(never)]
	extern "C" fn context_entry_point_function_pointer(transfer: Transfer) -> !
	{
		let mut type_safe_transfer = TypeSafeTransfer::<ParentInstructingChild<Self::ResumeArguments>, ChildOutcome<Self::Yields, Self::Complete>>::wrap(transfer);
		let start_child_arguments: Self::StartArguments = type_safe_transfer.start_child_arguments();

		let result =
		{
			let yielder = Yielder::new(&mut type_safe_transfer);
			catch_unwind(AssertUnwindSafe(|| Self::coroutine(start_child_arguments, yielder)))
		};

		type_safe_transfer.resume_drop_safe(ChildOutcome::Complete(result));
		unsafe { unreachable() }
	}
}
