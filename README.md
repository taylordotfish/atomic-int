atomic-int
==========

atomic-int provides atomics for additional integers, such as C/FFI types
like [`c_int`].

For integer types that are aliases of primitive integers that have built-in
Rust atomics, this crate simply re-exports those atomics. Otherwise, this
crate provides a spinlock-based fallback implementation with a compatible
API.

This crate also provides types that directly correspond with Rust’s
standard atomics, like [`AtomicU64`], with the difference that the fallback
implementation will similarly be used for any such atomics that are not
supported on a given platform. Thus, all atomics provided by this crate are
available on all platforms[^1] in some form—either the built-in or fallback
implementation.

Crate features
--------------

Types that directly correspond with Rust’s standard atomics like
[`AtomicU64`] are available with the feature `primitives` (enabled by
default).

Atomic C integer types like [`AtomicCInt`] and [`AtomicCUlong`] are
available with the feature `c` (enabled by default). For more granularity,
a separate feature exists for each C integer (e.g., `c_int` and `c_ulong`).

The spinlock-based fallback implementation can cause deadlocks with signal
handlers. To avoid this, enable the feature `signal`, which blocks incoming
signals while the lock is held. This feature is Unix-specific.

atomic-int can optionally depend on [`libc`]. If this dependency is
enabled, atomic-int will use the C integer types from [`libc`] instead of
[`core::ffi`]. This should not make a noticeable difference, but it can
decrease the minimum required Rust version, as C integer types were added
to [`core::ffi`] only in version 1.64. The feature `signal` always enables
`libc`.

This crate is `no_std` when `libc` is not enabled.

[^1]: As long as the platform supports [`AtomicBool`], which is required
      for the fallback implementation.

[`libc`]: https://docs.rs/libc/0.2
[`c_int`]: https://doc.rust-lang.org/stable/core/ffi/type.c_int.html
[`AtomicU64`]: https://docs.rs/atomic-int/0.1/atomic_int/type.AtomicU64.html
[`AtomicCInt`]: https://docs.rs/atomic-int/0.1/atomic_int/type.AtomicCInt.html
[`AtomicCUlong`]: https://docs.rs/atomic-int/0.1/atomic_int/type.AtomicCUlong.html
[`core::ffi`]: https://doc.rust-lang.org/stable/core/ffi/
[`AtomicBool`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicBool.html

Documentation
-------------

[Documentation is available on docs.rs.](https://docs.rs/atomic-int/0.1)

License
-------

atomic-int is licensed under version 2 of the Apache License. See
[LICENSE](LICENSE).

Contributing
------------

By contributing to atomic-int, you agree that your contribution may be used
according to the terms of atomic-int’s license.
