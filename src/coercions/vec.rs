use libc;
use num::ToPrimitive;
use sys;
use sys::VALUE;
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust};

impl<T> UncheckedValue<Vec<T>> for VALUE
    where VALUE: UncheckedValue<T>
{
    fn to_checked(self) -> CheckResult<Vec<T>> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_ARRAY) } {
            let array_len = unsafe { sys::RARRAY_LEN(self) };
            for i in 0..array_len {
                let val = unsafe { sys::rb_ary_entry(self, i as libc::c_long) };
                let _: CheckedValue<T> = try!(val.to_checked());
            }
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            Err(CString::new(format!("No implicit conversion from {} to String", "?")).unwrap())
        }
    }
}

impl<T> ToRust<Vec<T>> for CheckedValue<Vec<T>>
    where CheckedValue<T>: ToRust<T>, VALUE: UncheckedValue<T>
{
    fn to_rust(self) -> Vec<T> {
        let array_len = unsafe { sys::RARRAY_LEN(self.inner) };
        let mut vec = Vec::with_capacity(array_len.to_usize().unwrap());
        for i in 0..array_len {
            let val = unsafe { sys::rb_ary_entry(self.inner, i as libc::c_long) };
            // unwrap is okay because we already checked that each element is coercible to T above
            let el = val.to_checked().unwrap();
            vec.push(el.to_rust());
        }
        vec
    }
}
