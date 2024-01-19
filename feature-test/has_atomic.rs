/*
 * Copyright 2023 taylor.fish <contact@taylor.fish>
 *
 * This file is part of atomic-int.
 *
 * atomic-int is licensed under the Apache License, Version 2.0
 * (the "License"); you may not use atomic-int except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![no_std]
#[allow(unused_imports)]
use core::ffi;

pub trait HasAtomic {}

macro_rules! impl_has_atomic {
    ($int:ident, $bits:literal) => {
        #[cfg(target_has_atomic = $bits)]
        impl HasAtomic for $int {}
    };
}

impl_has_atomic!(u8, "8");
impl_has_atomic!(i8, "8");
impl_has_atomic!(u16, "16");
impl_has_atomic!(i16, "16");
impl_has_atomic!(u32, "32");
impl_has_atomic!(i32, "32");
impl_has_atomic!(u64, "64");
impl_has_atomic!(i64, "64");
impl_has_atomic!(usize, "ptr");
impl_has_atomic!(isize, "ptr");

macro_rules! impl_c_test {
    ($int:ident, $cfg:ident) => {
        #[cfg($cfg)]
        #[allow(non_camel_case_types)]
        pub struct $cfg<T: HasAtomic = ffi::$int>(T);
    };
}

impl_c_test!(c_char, test_has_c_char_atomic);
impl_c_test!(c_schar, test_has_c_schar_atomic);
impl_c_test!(c_uchar, test_has_c_uchar_atomic);
impl_c_test!(c_short, test_has_c_short_atomic);
impl_c_test!(c_ushort, test_has_c_ushort_atomic);
impl_c_test!(c_int, test_has_c_int_atomic);
impl_c_test!(c_uint, test_has_c_uint_atomic);
impl_c_test!(c_long, test_has_c_long_atomic);
impl_c_test!(c_ulong, test_has_c_ulong_atomic);
impl_c_test!(c_longlong, test_has_c_longlong_atomic);
impl_c_test!(c_ulonglong, test_has_c_ulonglong_atomic);
