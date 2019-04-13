# context-coroutine

Provides coroutines using the trait `Coroutine`.

Coroutines use a separate, special stack.

Implement this trait and then call `Coroutine::start_coroutine()`, passing in start arguments and a source of memory for the stack and heap.
Coroutines can use a switchable allocator, providing a straightforward way to restrict the amount of dynamic memory they can access and to ensure they only use thread-local memory.

For a simple coroutine, use the stack `stacks::ProtectedStack`.

This crate was originally a simple set of extensions to the [context](https://github.com/zonyitoo/context-rs) crate to provide stackful coroutines.
The developers are not associated with the authors of [context](https://github.com/zonyitoo/context-rs) but are extremely grateful for the work they've put into to a superb piece of code.


## Licensing

The license for this project is MIT.
