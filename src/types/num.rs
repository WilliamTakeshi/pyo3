use crate::{ffi, ffi_ptr_ext::FfiPtrExt, instance::Bound, PyAny, Python};
use num::ToPrimitive;

use super::any::PyAnyMethods;

// use crate::{ffi, ffi_ptr_ext::FfiPtrExt, instance::Bound, PyAny, Python};

/// Represents a Python `int` object.
///
/// You can usually avoid directly working with this type
/// by using [`ToPyObject`](crate::conversion::ToPyObject)
/// and [`extract`](super::PyAnyMethods::extract)
/// with the primitive Rust integer types.
#[repr(transparent)]
pub struct PyLong(PyAny);

pyobject_native_type_core!(PyLong, pyobject_native_static_type_object!(ffi::PyLong_Type), #checkfunction=ffi::PyLong_Check);

impl PyLong {
    /// Creates a new `PyLong` from a C `long` integer.
    /// # Returns
    /// A [`Bound`] reference to a `PyLong` object representing the given `val`.
    #[inline]
    pub fn new_bound<T: ToPrimitive>(py: Python<'_>, val: T) -> Bound<'_, Self> {
        let val = val.to_i64().unwrap();
        unsafe {
            ffi::PyLong_FromLongLong(val)
                .assume_owned(py)
                .downcast_into_unchecked()
        }
    }
}

/// Implementation of functionality for [`PyLong`].
///
/// These methods are defined for the `Bound<'py, PyLong>` smart pointer, so to use method call
/// syntax these methods are separated into a trait, because stable Rust does not yet support
/// `arbitrary_self_types`.
#[doc(alias = "PyLong")]
pub trait PyLongMethods<'py> {
    /// Gets the value of this int.
    fn value(&self) -> i64;
}

impl<'py> PyLongMethods<'py> for Bound<'py, PyLong> {
    fn value(&self) -> i64 {
        unsafe {
            // Safety: self is PyLong object
            ffi::PyLong_AsLongLong(self.as_ptr())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PyLong;
    use super::PyLongMethods;
    use crate::Python;

    #[test]
    fn test_pylong_value() {
        Python::with_gil(|py| {
            let v = 123 as i64;
            let obj = PyLong::new_bound(py, v);
            assert_eq!(v, obj.value());

            let v = i64::MIN;
            let obj = PyLong::new_bound(py, v);
            assert_eq!(v, obj.value());

            let v = i64::MAX;
            let obj = PyLong::new_bound(py, v);
            assert_eq!(v, obj.value());

            let v = i32::MIN;
            let obj = PyLong::new_bound(py, v);
            assert_eq!(v as i64, obj.value());

            let v = i32::MAX;
            let obj = PyLong::new_bound(py, v);
            assert_eq!(v as i64, obj.value());

            let v = u64::MIN;
            let obj = PyLong::new_bound(py, v);
            assert_eq!(v as i64, obj.value());
        });
    }
}
