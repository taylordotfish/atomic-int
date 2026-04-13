Changelog
=========

0.1.5
-----

* Fixed behavior with versions of Rust before 1.64. Normally, 1.64 is this
  crate's MSRV, but older versions of Rust can be used if the `libc` feature is
  enabled. However, this crate would erroneously use fallback types for all C
  atomics in this case due to code in `feature-test/has_atomic.rs` that
  required newer versions of Rust.
* Documented that `AtomicBool` must support compare-and-swap operations.
* Documented that the fallback implementation is also used when the standard
  library type exists but doesn't provide compare-and-swap operations.
* Made the `signal` feature a no-op on non-Unix-like systems. Previously, it
  would cause compilation errors on those systems.
* Added new feature `force-fallback` for development purposes, to force all
  atomics to use the fallback implementation. As the standard library evolves,
  some functions might not yet be present in the fallback implementation (and
  some, like `from_ptr`, cannot be implemented), so this feature can help check
  whether a crate is unintentionally using any unsupported functions.
* Replaced build script with a metaprogramming-based approach.

0.1.4
-----

* In the fallback implementation, when an atomic operation is requested with
  `SeqCst` ordering, the spinlock is now both acquired and released with
  `SeqCst` ordering. Previously, it was acquired with `Acquire` ordering and
  released with `SeqCst` ordering. It is not clear that this change is
  necessary for correctness, but given that the performance impact is likely
  negligible, it seems reasonable to make this change for the added assurance.

0.1.3
-----

* Fixed potential compilation errors by unconditionally using the fallback
  implementation for 128-bit atomics. 128-bit atomics have sometimes been
  present in the standard library, but only behind an unstable feature, which
  would cause errors in stable Rust and in any crate that didn't enable the
  feature.

0.1.2
-----

* Added error handling when calling libc functions in the signal-blocking code
  (crate feature `signal`).

0.1.1
-----

* Updated the fallback implementation to honor the failure ordering in
  `compare_exchange`. This only affects cases where `failure` is `SeqCst` and
  `success` isn't.

0.1.0
-----

Initial release.
