use super::{Argument, ArgumentFlag};

pub mod internal {
    use super::*;

    pub struct ArgValue<T>(pub T);

    pub trait IntoArgument<'a> {
        fn into_arg(self, flag: ArgumentFlag) -> Argument<'a>;
    }

    impl<T: Copy + 'static> IntoArgument<'static> for ArgValue<T> {
        fn into_arg(self, flag: ArgumentFlag) -> Argument<'static> {
            Argument::from_value(self.0, flag)
        }
    }

    impl<'a, T: 'static> IntoArgument<'a> for &'a T {
        fn into_arg(self, flag: ArgumentFlag) -> Argument<'a> {
            Argument::from_ref(self, flag)
        }
    }

    impl<'a, T: 'static> IntoArgument<'a> for &'a mut T {
        fn into_arg(self, flag: ArgumentFlag) -> Argument<'a> {
            Argument::from_mut(self, flag)
        }
    }

    impl<'a, T: 'static> IntoArgument<'a> for &'a [T] {
        fn into_arg(self, flag: ArgumentFlag) -> Argument<'a> {
            Argument::from_slice(self, flag)
        }
    }

    impl<'a, T: 'static> IntoArgument<'a> for &'a mut [T] {
        fn into_arg(self, flag: ArgumentFlag) -> Argument<'a> {
            Argument::from_mut_slice(self, flag)
        }
    }

    pub trait UnsafeIntoArgument<'a> {
        /// Creates an `Argument` from a raw pointer.
        ///
        /// # Safety
        ///
        /// The caller must guarantee that the raw pointer `self` is valid for the lifetime `'a`
        /// and that its usage adheres to Rust's aliasing rules.
        ///
        /// Specifically, for the entire duration of the lifetime `'a`:
        ///
        /// 1.  **Validity**: The pointer must be non-null and point to a single, properly
        ///     initialized value of type `T`. It must not be dangling.
        /// 2.  **Alignment**: The pointer must be properly aligned for the type `T`.
        /// 3.  **Lifetime and Access**:
        ///     *   If `self` is a `*const T`, the memory it points to must be **valid for reads**
        ///         for the lifetime `'a`. During this time, the memory must not be mutated
        ///         by any other pointer (e.g., a concurrent `*mut T`). It can, however, be
        ///         read through other shared references or pointers.
        ///     *   If `self` is a `*mut T`, the memory it points to must be **valid for both reads
        ///         and writes** for the lifetime `'a`. Crucially, during this time, **no other
        ///         pointer or reference (read or write) may access this memory**. The pointer
        ///         must have exclusive access, upholding the same aliasing rules as a `&'a mut T`.
        ///
        /// Failure to uphold any of these guarantees will result in **undefined behavior**.
        unsafe fn into_arg(self, flag: ArgumentFlag) -> Argument<'a>;
    }

    impl<'a, T: 'static> UnsafeIntoArgument<'a> for *const T {
        unsafe fn into_arg(self, flag: ArgumentFlag) -> Argument<'a> {
            unsafe { Argument::from_ptr(self, flag) }
        }
    }

    impl<'a, T: 'static> UnsafeIntoArgument<'a> for *mut T {
        unsafe fn into_arg(self, flag: ArgumentFlag) -> Argument<'a> {
            unsafe { Argument::from_mut_ptr(self, flag) }
        }
    }
}

#[macro_export]
macro_rules! arg_flag {
    ($($flags:ident),*) => {
        {
            let mut temp_flags = $crate::ipc::message::ArgumentFlag::default();
            $(
                temp_flags.insert($crate::ipc::message::ArgumentFlag::$flags);
            )*
            temp_flags
        }
    };
}

#[macro_export]
macro_rules! arg {
    // --- empty argument ---
    () => {
        $crate::ipc::message::Argument::empty()
    };

    // --- `val` variants ---
    (val($value:expr), flag($($flags:ident),*)) => {
        $crate::ipc::message::macros::internal::IntoArgument::into_arg(
            $crate::ipc::message::macros::internal::ArgValue($value),
            $crate::arg_flag!($($flags),*)
        )
    };
    (val($value:expr)) => {
        $crate::ipc::message::macros::internal::IntoArgument::into_arg(
            $crate::ipc::message::macros::internal::ArgValue($value),
            $crate::ipc::message::ArgumentFlag::default()
        )
    };

    // --- `ref` variants ---
    (ref($value:expr), flag($($flags:ident),*)) => {
        $crate::ipc::message::macros::internal::IntoArgument::into_arg(
            $value,
            $crate::arg_flag!($($flags),*)
        )
    };
    (ref($value:expr)) => {
        $crate::ipc::message::macros::internal::IntoArgument::into_arg(
            $value,
            $crate::ipc::message::ArgumentFlag::default()
        )
    };

    // --- `ptr` variants ---
    (ptr($value:expr), flag($($flags:ident),*)) => {
        $crate::ipc::message::macros::internal::UnsafeIntoArgument::into_arg(
            $value,
            $crate::arg_flag!($($flags),*)
        )
    };
    (ptr($value:expr)) => {
        $crate::ipc::message::macros::internal::UnsafeIntoArgument::into_arg(
            $value,
            $crate::ipc::message::ArgumentFlag::default()
        )
    };
}

#[macro_export]
macro_rules! request {
    // --- no argument --
    ($method_id:expr) => {
        $crate::ipc::message::Request::empty($method_id)
    };

    // --- multiple arguments --
    ($method_id:expr, [ $($arg:expr),* $(,)? ]) => {
        $crate::ipc::message::Request::with_args(
            $method_id,
            // The magic is here: we just use the standard vec! macro
            // to collect all the provided expressions.
            vec![$($arg),*]
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_macro() {
        let value = 42i32;
        let mut data_vec = vec![10u8, 20, 30];
        let mut mutable_scalar = 100i64;

        let arg_empty = arg!();
        println!("arg!(): {:#?}", arg_empty);
        assert_eq!(arg_empty.type_size(), 0);
        assert!(arg_empty.is_empty());
        println!("");

        // no flag
        let arg_val_no_flag = arg!(val(value));
        println!("arg!(val(...): {:#?}", arg_val_no_flag);
        assert_eq!(arg_val_no_flag.type_size(), size_of::<i32>());
        assert_eq!(arg_val_no_flag.type_align(), align_of::<i32>());
        assert_eq!(arg_val_no_flag.flag(), ArgumentFlag::default());
        assert_eq!(arg_val_no_flag.downcast::<i32>().unwrap(), 42);
        println!("");

        // multiple flags
        let arg_val_with_flags = arg!(val(value), flag(ARG_IN, ARG_OUT));
        println!("arg!(val(...): {:#?}", arg_val_with_flags);
        assert!(arg_val_with_flags.flag().contains(ArgumentFlag::ARG_IN));
        assert!(arg_val_with_flags.flag().contains(ArgumentFlag::ARG_OUT));
        println!("");

        // &T
        let arg_ref = arg!(ref(&mutable_scalar), flag(ARG_IN));
        println!("arg!(ref(...): {:#?}", arg_ref);
        assert_eq!(arg_ref.type_size(), size_of::<i64>());
        assert_eq!(arg_ref.downcast_ref::<i64>().unwrap(), &100);
        println!("");

        // &[T]
        let arg_slice = arg!(ref(data_vec.as_slice()));
        println!("arg!(ref(...): {:#?}", arg_slice);
        assert_eq!(arg_slice.total_size(), size_of::<u8>() * 3);
        assert_eq!(arg_slice.downcast_slice::<u8>().unwrap(), &[10, 20, 30]);
        println!("");

        // &mut [T]
        let arg_mut_slice = arg!(ref(data_vec.as_mut_slice()), flag(ARG_OUT));
        println!("arg!(ref(...): {:#?}", arg_mut_slice);
        let slice = unsafe { arg_mut_slice.downcast_mut_slice::<u8>().unwrap() };
        slice[1] = 25;
        assert_eq!(data_vec, vec![10, 25, 30]);
        println!("");

        let const_ptr: *const u8 = data_vec.as_ptr();
        let mut_ptr: *mut i64 = &mut mutable_scalar;

        // *const T
        let arg_const_ptr = unsafe { arg!(ptr(const_ptr), flag(ARG_VIRT)) };
        println!("arg!(ptr(...)): {:#?}", arg_const_ptr);
        assert!(arg_const_ptr.flag().contains(ArgumentFlag::ARG_VIRT));
        assert_eq!(*arg_const_ptr.downcast_ref::<u8>().unwrap(), 10);
        println!("");

        // *mut T
        let arg_mut_ptr = unsafe { arg!(ptr(mut_ptr)) };
        println!("arg!(ptr(...)): {:#?}", arg_mut_ptr);
        let ptr_back = unsafe { arg_mut_ptr.downcast_mut::<i64>().unwrap() };
        *ptr_back = 200;
        assert_eq!(mutable_scalar, 200);
    }

    #[test]
    fn test_request_macro() {
        let my_data = [1u8, 2, 3, 4];
        let mut output_buffer = [0u8; 4];

        let empty_req = request!(102);
        assert_eq!(empty_req.method_id, 102);
        assert!(empty_req.arg_list.is_empty());

        let req = request!(
            101,
            [
                arg!(val(42i32)),
                arg!(ref(&my_data), flag(ARG_OUT, ARG_VIRT)),
                unsafe { arg!(ptr(output_buffer.as_mut_ptr())) }
            ]
        );

        println!("Request: {:#?}", req);
        assert_eq!(req.method_id, 101);
        assert_eq!(req.arg_list.len(), 3);
    }
}
