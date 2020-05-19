// This file is part of context-coroutine. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT. No part of context-coroutine, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-coroutine. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-coroutine/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(naked_functions)]


//! # context-coroutine
//!
//! Provides coroutines using the trait `Coroutine`.
//!
//! Coroutines use a separate, special stack.
//!
//! Implement this trait and then call `Coroutine::start_coroutine()`, passing in start arguments and a source of memory for the stack and heap.
//! Coroutines can use a switchable allocator, providing a straightforward way to restrict the amount of dynamic memory they can access and to ensure they only use thread-local memory.
//!
//! For a simple coroutine, use the stack `stacks::ProtectedStack`.
//!
//! This crate was originally a simple set of extensions to the [context](https://github.com/zonyitoo/context-rs) crate to provide stackful coroutines.
//! The developers are not associated with the authors of [context](https://github.com/zonyitoo/context-rs) but are extremely grateful for the work they've put into to a superb piece of code.
//!
//!
//! ## Licensing
//!
//! The license for this project is MIT.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use self::context::*;
use self::stacks::*;
use const_fn_assert::cfn_debug_assert;
use context_allocator::allocators::global::*;
use context_allocator::memory_sources::*;
use likely::*;
use linux_support::memory::huge_pages::DefaultPageSizeAndHugePageSizes;
use linux_support::memory::mapping::MappedMemory;
use magic_ring_buffer::*;
use magic_ring_buffer::memory_sizes::MemorySize;
use std::alloc::AllocErr;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::intrinsics::unreachable;
use std::marker::PhantomData;
use std::mem::size_of;
#[allow(deprecated)] use std::mem::uninitialized;
use std::num::NonZeroU64;
use std::num::NonZeroUsize;
use std::panic::*;
use std::ptr::NonNull;
use std::ptr::read;
use std::ptr::write;
use std::thread;


include!("ChildOutcome.rs");
include!("Coroutine.rs");
include!("CoroutineGenerationCounter.rs");
include!("CoroutineInstance.rs");
include!("CoroutineInstanceAllocator.rs");
include!("CoroutineInstanceHandle.rs");
include!("CoroutineInstancePointer.rs");
include!("CoroutineManager.rs");
include!("ParentInstructingChild.rs");
include!("ResumeOutcome.rs");
include!("StartOutcome.rs");
include!("TaggedRelativePointerToData.rs");
include!("Yielder.rs");


/// Context; derived from `Boost.Context` and [context-rs](https://github.com/zonyitoo/context-rs).
///
/// Use the `TypeSafeTransfer` struct to work with contexts (or the lower-level `Transfer`).
pub mod context;


/// Stack implementations.
pub mod stacks;
