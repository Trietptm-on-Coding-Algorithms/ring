// Copyright 2015-2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Safe, fast, small crypto using Rust with BoringSSL's cryptography
//! primitives.
//!
//! <code>git clone https://github.com/briansmith/ring</code>
//!
//! # Feature Flags
//!
//! <table>
//! <tr><th>Feature
//!     <th>Description
//! <tr><td><code>dev_urandom_fallback (default)</code>
//!     <td>This is only applicable to Linux. On Linux, by default,
//!         <code>ring::rand::SystemRandom</code> will fall back to reading
//!         from <code>/dev/urandom</code> if the <code>getrandom()</code>
//!         syscall isn't supported at runtime. When the
//!         <code>dev_urandom_fallback</code> feature is disabled, such
//!         fallbacks will not occur. See the documentation for
//!         <code>rand::SystemRandom</code> for more details.
//! <tr><td><code>rsa_signing</code>
//!     <td>Enable RSA signing (<code>RSAKeyPair</code> and related things).
//! </table>

#![doc(html_root_url="https://briansmith.org/rustdoc/")]

#![allow(
    legacy_directory_ownership,
    missing_copy_implementations,
    missing_debug_implementations,
    unknown_lints,
    unsafe_code,
    // TODO: Deny `unused_qualifications` after
    // https://github.com/rust-lang/rust/issues/37345 is fixed.
    unused_qualifications,
)]
#![deny(
    const_err,
    dead_code,
    deprecated,
    exceeding_bitshifts,
    fat_ptr_transmutes,
    improper_ctypes,
    missing_docs,
    mutable_transmutes,
    no_mangle_const_items,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    path_statements,
    plugin_as_library,
    private_no_mangle_fns,
    private_no_mangle_statics,
    stable_features,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unknown_crate_types,
    unreachable_code,
    unstable_features,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_extern_crates,
    unused_features,
    unused_imports,
    unused_import_braces,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_results,
    unused_unsafe,
    unused_variables,
    variant_size_differences,
    warnings,
    while_true,
)]

#![no_std]

#![cfg_attr(feature = "internal_benches", allow(unstable_features))]
#![cfg_attr(feature = "internal_benches", feature(test))]

extern crate libc;

#[cfg(feature = "internal_benches")]
extern crate test as bench;

#[cfg(any(target_os = "redox",
          all(unix,
              any(not(target_os = "linux"),
                  feature = "dev_urandom_fallback"))))]
#[macro_use]
extern crate lazy_static;

// `ring::test` uses the formatting & printing stuff in non-test mode.
#[cfg_attr(not(test), macro_use(format, print, println))]
#[cfg_attr(test, macro_use(format, print, println, vec))]
extern crate std;

extern crate untrusted;

#[macro_use]
mod bssl;

#[macro_use]
mod polyfill;

#[path = "aead/aead.rs"]
pub mod aead;

pub mod agreement;

#[cfg(feature = "use_heap")]
mod bits;

mod c;
mod chacha;
pub mod constant_time;

#[doc(hidden)]
pub mod der;

#[path = "digest/digest.rs"]
pub mod digest;

#[path = "ec/ec.rs"]
mod ec;

pub mod error;
pub mod hkdf;
pub mod hmac;
mod init;
mod limb;
pub mod pbkdf2;
mod poly1305;
pub mod rand;

#[cfg(feature = "use_heap")]
#[path = "rsa/rsa.rs"]
mod rsa;

// Really a private method; only has public visibility so that C compilation
// can see it.
#[cfg(feature = "use_heap")]
#[doc(hidden)]
pub use rsa::GFp_rand_mod;

pub mod signature;

#[cfg(any(feature = "use_heap", test))]
pub mod test;

mod private {
    /// Traits that are designed to only be implemented internally in *ring*.
    //
    // Usage:
    // ```
    // use private;
    //
    // pub trait MyType : private::Private {
    //     // [...]
    // }
    //
    // impl private::Private for MyType { }
    // ```
    pub trait Private {}
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "use_heap")]
    bssl_test_rng!(test_bn, bssl_bn_test_main);

    bssl_test!(test_constant_time, bssl_constant_time_test_main);
}
