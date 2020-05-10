// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// A protected stack backed by mmap'd memory.
///
/// Not efficient for many, many coroutines.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtectedStack
{
	top_including_guard_page: usize,
	size_including_guard_page: usize,

	mapped_memory: MappedMemory,
}

impl Stack for ProtectedStack
{
	#[inline(always)]
	fn bottom(&self) -> StackPointer
	{
		// Yes, this is correct.
		// On x86-64 and all other systems on Linux (the obsolete PA-RISC being the exception), stacks grow downwards, so the concepts of top and bottom are the opposite to high and low!
		(self.top_including_guard_page + self.size_including_guard_page) as StackPointer
	}
}

impl ProtectedStack
{
	/// Allocate a `size` in bytes.
	#[inline(always)]
	pub fn allocate(size: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, CreationError>
	{
		let page_size = PageSize::current().size_in_bytes().get();

		let size_including_guard_page_but_might_be_bigger_than_maximum_stack_size = unsafe { NonZeroU64::new_unchecked(PageSize::current().non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(size).get() + page_size) };
		let size_including_guard_page = min(size_including_guard_page_but_might_be_bigger_than_maximum_stack_size, Self::maximum_stack_size());

		let mapped_memory = MappedMemory::anonymous(size_including_guard_page, AddressHint::any(), Protection::ReadWrite, Sharing::Private, None, false, false, defaults)?;

		// Create a guard page at the top.
		mapped_memory.change_protection_range(ExtendedProtection::Inaccessible, 0 .. (page_size as usize)).expect("No good reason to fail");

		Ok
		(
			Self
			{
				top_including_guard_page: mapped_memory.virtual_address().into(),
				size_including_guard_page: size_including_guard_page.get() as usize,
				mapped_memory,
			}
		)
	}

	#[inline(always)]
	fn maximum_stack_size() -> NonZeroU64
	{
		#[allow(deprecated)]
		#[inline(always)]
		fn uncached_maximum_stack_size() -> NonZeroU64
		{
			let resource_limit = ResourceName::MaximumSizeOfProcessStackInBytes.get();
			let maximum = resource_limit.hard_limit();
			unsafe { NonZeroU64::new_unchecked(maximum.value()) }
		}

		static MaximumStackSize: AtomicU64 = AtomicU64::new(0);
		let potential_maximum_stack_size = MaximumStackSize.load(Relaxed);
		if unlikely!(potential_maximum_stack_size == 0)
		{
			let maximum_stack_size = uncached_maximum_stack_size();
			MaximumStackSize.store(maximum_stack_size.get(), Relaxed);
			maximum_stack_size
		}
		else
		{
			unsafe { NonZeroU64::new_unchecked(potential_maximum_stack_size) }
		}
	}
}
