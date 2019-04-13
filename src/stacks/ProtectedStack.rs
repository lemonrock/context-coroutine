// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


/// A protected stack.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtectedStack
{
	top_including_guard_page: usize,
	size_including_guard_page: usize,
}

impl Drop for ProtectedStack
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { munmap(self.top_including_guard_page as *mut _, self.size_including_guard_page) };
	}
}

impl Stack for ProtectedStack
{
	#[inline(always)]
	fn bottom(&self) -> StackPointer
	{
		// Yes, this is correct.
		// On x86-64 and most other systems, stacks grow downwards, so the concepts of top and bottom are the opposite to high and low!
		(self.top_including_guard_page + self.size_including_guard_page) as StackPointer
	}
}

impl ProtectedStack
{
	/// Allocate a `size` in bytes.
	#[inline(always)]
	pub fn allocate(size: usize) -> Result<Self, io::Error>
	{
		const NoFileDescriptor: c_int = -1;
		const NoOffset: off_t = 0;

		let page_size = Self::page_size();
		let size_including_guard_page_but_might_be_bigger_than_maximum_stack_size = Self::round_up_to_page_size(size, page_size) + page_size;
		let size_including_guard_page = min(size_including_guard_page_but_might_be_bigger_than_maximum_stack_size, Self::maximum_stack_size());

		#[cfg(not(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd", target_os = "linux", target_os = "openbsd")))] const MAP_STACK: i32 = 0;

		let top_including_guard_page = unsafe { mmap(null_mut(), size_including_guard_page, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON | MAP_STACK | MAP_NORESERVE, NoFileDescriptor, NoOffset) };
		if unlikely!(top_including_guard_page == MAP_FAILED)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			let result = unsafe { mprotect(top_including_guard_page, page_size, PROT_NONE) };
			if likely!(result == 0)
			{;
				Ok
				(
					Self
					{
						top_including_guard_page: (top_including_guard_page as usize),
						size_including_guard_page,
					}
				)
			}
			else if likely!(result == -1)
			{
				Err(io::Error::last_os_error())
			}
			else
			{
				unreachable!()
			}
		}
	}

	#[inline(always)]
	fn round_up_to_page_size(size: usize, page_size: usize) -> usize
	{
		(size + page_size - 1) & !(page_size - 1)
	}

	#[inline(always)]
	fn page_size() -> usize
	{
		// `getpagesize()` is faster than `sysconf(_SC_PAGESIZE)` on musl libc systems; on most systems, it's a hardcoded constant.
		// On ARM and a few others, it's a the value of a global static that never changes.
		(unsafe { getpagesize() }) as usize
	}

	#[inline(always)]
	fn maximum_stack_size() -> usize
	{
		#[inline(always)]
		fn uncached_maximum_stack_size() -> usize
		{
			let mut limit = unsafe { uninitialized()};
			let result = unsafe { getrlimit(RLIMIT_STACK, &mut limit) };

			if likely!(result == 0)
			{
				let maximum = limit.rlim_max;
				if maximum == RLIM_INFINITY || maximum > (::std::usize::MAX as rlim_t)
				{
					::std::usize::MAX
				}
				else
				{
					maximum as usize
				}
			}
			else if likely!(result == -1)
			{
				panic!("getrlimit() failed with `{:?}`", io::Error::last_os_error())
			}
			else
			{
				unreachable!()
			}
		}

		use self::Ordering::Relaxed;

		static MaximumStackSize: AtomicUsize = AtomicUsize::new(0);
		let potential_maximum_stack_size = MaximumStackSize.load(Relaxed);
		if unlikely!(potential_maximum_stack_size == 0)
		{
			let maximum_stack_size = uncached_maximum_stack_size();
			MaximumStackSize.store(maximum_stack_size, Relaxed);
			maximum_stack_size
		}
		else
		{
			potential_maximum_stack_size
		}
	}
}
