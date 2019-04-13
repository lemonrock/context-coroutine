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


//! #context-coroutine
//!
//! Provides coroutines using the trait `Coroutine`.
//!
//! Coroutines use a separate, special stack.
//!
//! Implement this trait and then call `Coroutine::start_coroutine()`, passing in start arguments and a stack.
//!
//! For a simple coroutine, use the stack `stacks::ProtectedStack`.
//!
//! The module `context` provides lower-level logic to use as a building block for things other than coroutines, for example, fibres.
//!
//! This crate was originally a simple set of extensions to the [context](https://github.com/zonyitoo/context-rs) crate to provide stackful coroutines.
//! The developers are not associated with the authors of [context](https://github.com/zonyitoo/context-rs) but are extremely grateful for the work they've put into to a superb piece of code.


extern crate context_allocator;
extern crate libc;
extern crate libc_extra;
#[macro_use] extern crate likely;


use self::context::*;
use self::stacks::*;
use ::context_allocator::*;
use ::context_allocator::arena_memory_source::*;
use ::context_allocator::extensions::*;
use ::context_allocator::global::*;
use ::context_allocator::mmap::*;
use ::std::alloc::AllocErr;
use ::std::intrinsics::unreachable;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::num::NonZeroUsize;
use ::std::panic::*;
use ::std::ptr::NonNull;
use ::std::thread;


include!("ChildOutcome.rs");
include!("Coroutine.rs");
include!("CoroutineMemory.rs");
include!("CoroutineMemorySource.rs");
include!("CoroutineInstance.rs");
include!("ParentInstructingChild.rs");
include!("ResumeOutcome.rs");
include!("StartedCoroutineInstance.rs");
include!("StartOutcome.rs");
include!("Yielder.rs");


/// Context; derived from `Boost.Context` and [context-rs](https://github.com/zonyitoo/context-rs).
///
/// Use the `TypeSafeTransfer` struct to work with contexts (or the lower-level `Transfer`).
pub mod context;


/// Stack implementations.
pub mod stacks;
