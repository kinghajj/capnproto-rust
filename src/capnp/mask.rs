/*
 * Copyright (c) 2013, David Renshaw (dwrenshaw@gmail.com)
 *
 * See the LICENSE file in the capnproto-rust root directory.
 */

pub trait Mask {
    fn mask(value : Self, mask : Self) -> Self;
}

// There's got to be a way to use a standard trait like Bitwise here,
// but I keep getting "conflicting implementation" errors.

macro_rules! int_mask(
    ($t:ident) => (
        impl Mask for $t {
            #[inline]
            fn mask(value : $t, mask : $t) -> $t {
                value ^ mask
            }
        }
    )
);

int_mask!(i8);
int_mask!(i16);
int_mask!(i32);
int_mask!(i64);
int_mask!(u8);
int_mask!(u16);
int_mask!(u32);
int_mask!(u64);

impl Mask for f32 {
    #[inline]
    fn mask(value : f32, mask : f32) -> f32 {
        use std;
        unsafe {
            let v : u32 = std::mem::transmute(value);
            let m : u32 = std::mem::transmute(mask);
            std::mem::transmute(v ^ m)
        }
    }
}

impl Mask for f64 {
    #[inline]
    fn mask(value : f64, mask : f64) -> f64 {
        use std;
        unsafe {
            let v : u64 = std::mem::transmute(value);
            let m : u64 = std::mem::transmute(mask);
            std::mem::transmute(v ^ m)
        }
    }
}
