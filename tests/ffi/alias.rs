// Rustfmt mangles the extern type alias.
// https://github.com/rust-lang/rustfmt/issues/4159
#[rustfmt::skip]
#[cxx::bridge(namespace = tests)]
pub mod ffi {
    type Shared = crate::ffi::Shared;
    type Enum = crate::ffi::Enum;

    extern "C" {
        include!("cxx-test-suite/tests.h");
        // Need complete C++ type declaration generated by other bridge for aliased shared types.
        include!("cxx-test-suite/lib.rs.h");

        type C = crate::ffi::C;

        // TODO(https://github.com/dtolnay/cxx/pull/298): The alias prefix can be removed once these
        // are in their own namespace.
        fn alias_c_return_shared() -> Shared;
        fn alias_c_return_unique_ptr() -> UniquePtr<C>;
        fn alias_c_return_ref(shared: &Shared) -> &usize;
        fn alias_c_return_mut(shared: &mut Shared) -> &mut usize;
        fn alias_c_return_enum(n: u16) -> Enum;
        fn alias_c_return_unique_ptr_shared() -> UniquePtr<Shared>;
        fn alias_c_return_unique_ptr_vector_shared() -> UniquePtr<CxxVector<Shared>>;

        fn alias_c_take_shared(shared: Shared);
        fn alias_c_take_box_shared(shared: Box<Shared>);
        fn alias_c_take_unique_ptr(c: UniquePtr<C>);
        fn alias_c_take_unique_ptr_shared(s: UniquePtr<Shared>);
        fn alias_c_take_unique_ptr_vector_shared(v: UniquePtr<CxxVector<Shared>>);
        fn alias_c_take_rust_vec_shared(v: Vec<Shared>);
        fn alias_c_take_enum(e: Enum);

        fn get3(self: &C) -> usize;
        fn set3(self: &mut C, n: usize) -> usize;
    }

    extern "Rust" {
        // TODO(https://github.com/dtolnay/cxx/pull/298): The alias prefix can be removed once these
        // are in their own namespace.
        fn alias_r_return_shared() -> Shared;
        fn alias_r_return_ref(shared: &Shared) -> &usize;
        fn alias_r_return_mut(shared: &mut Shared) -> &mut usize;
        fn alias_r_return_enum(n: u32) -> Enum;

        fn alias_r_take_shared(shared: Shared);
        fn alias_r_take_enum(e: Enum);
    }
}

use crate::r_return_enum as alias_r_return_enum;
use crate::r_return_mut as alias_r_return_mut;
use crate::r_return_ref as alias_r_return_ref;
use crate::r_return_shared as alias_r_return_shared;
use crate::r_take_enum as alias_r_take_enum;
use crate::r_take_shared as alias_r_take_shared;
