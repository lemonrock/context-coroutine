// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



/// Use it like this:-
/// ```rust
/// use context_coroutine::CoroutineManagerIndex;
/// use linux_support::file_descriptors::socket::c::sockaddr_in;
/// use linux_support::file_descriptors::socket::c::sockaddr_in6;
///
/// struct SomeCoroutineManagerFields
/// {
/// 	accept_ipv4: CoroutineManager<HeapSize, StackSize, GTACSA, AcceptCoroutine<sockaddr_in>, ()>,
/// 	accept_ipv6: CoroutineManager<HeapSize, StackSize, GTACSA, AcceptCoroutine<sockaddr_in6>, ()>,
/// }
///
/// fn choose(coroutine_manager_index: CoroutineManagerIndex, some_coroutine_manager_fields: &mut SomeCoroutineManagerFields, arg1: usize, arg2: String)
/// {
/// 	choose_coroutine_manager!
/// 	{
/// 		coroutine_manager_index,
/// 		(arg1, arg2),
/// 		callback_on_coroutine_manager_mutable_instance,
/// 		some_coroutine_manager_fields,
/// 		0 => accept_ipv4 @ SomeModule::callback,
/// 		1 => accept_ipv6 @ local_function,
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! choose_coroutine_manager
{
    ($actual_coroutine_manager_index: expr, $callback: ident, $arguments: expr, $coroutine_manager_fields: expr, $($coroutine_manager_index: expr => $coroutine_manager_field_name: ident,)* ) =>
    {
        match $actual_coroutine_manager_index
        {
            $(
                CoroutineManagerIndex($coroutine_manager_index) =>
                {
                	let coroutine_manager = &mut $coroutine_manager_fields.$coroutine_manager_field_name;
                	
                	debug_assert!(coroutine_manager.has_index(CoroutineManagerIndex($coroutine_manager_index)));
                
                	coroutine_manager.$callback($arguments)
                }
            )*
            _ => unreachable_code(format_args!("Unregistered")),
        }
    };
}
