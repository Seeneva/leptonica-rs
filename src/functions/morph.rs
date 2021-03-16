// Copyright 2021 Sergei Solodovnikov
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Docstrings for leptonica ffi functions was copied from: 
//      https://github.com/DanBloomberg/leptonica/blob/master/src/morph.c
//
// These docstrings license:
//      BSD 2-Clause License
//      Copyright (C) 2001 Leptonica. 
//      All rights reserved.
use std::convert::TryInto;

use super::{AsLeptonicaPtr, Pix, Result};

pub trait Morph: AsLeptonicaPtr {
    fn dilate_brick(&mut self, h_size: i32, v_size: i32) -> Result<()> {
        dilate_brick_inner(self.as_ptr(), self.as_ptr(), h_size, v_size).map(|pix| {
            std::mem::forget(pix);

            ()
        })
    }

    fn dilate_brick_into_copy(&self, h_size: i32, v_size: i32) -> Result<Pix> {
        dilate_brick_inner(self.as_ptr(), std::ptr::null_mut(), h_size, v_size)
    }

    fn erode_brick(&mut self, h_size: i32, v_size: i32) -> Result<()> {
        erode_brick_inner(self.as_ptr(), self.as_ptr(), h_size, v_size).map(|pix|{
            std::mem::forget(pix);

            ()
        })
    }

    fn erode_brick_into_copy(&self, h_size: i32, v_size: i32) -> Result<Pix> {
        erode_brick_inner(self.as_ptr(), std::ptr::null_mut(), h_size, v_size)
    }
}

fn dilate_brick_inner(
    pix_s: *mut leptonica_sys::PIX,
    pix_d: *mut leptonica_sys::PIX,
    h_size: i32,
    v_size: i32,
) -> Result<Pix> {
    unsafe { leptonica_sys::pixDilateBrick(pix_d, pix_s, h_size as _, v_size as _) }.try_into()
}

fn erode_brick_inner(
    pix_s: *mut leptonica_sys::PIX,
    pix_d: *mut leptonica_sys::PIX,
    h_size: i32,
    v_size: i32,
) -> Result<Pix> {
    unsafe { leptonica_sys::pixErodeBrick(pix_d, pix_s, h_size as _, v_size as _) }.try_into()
}

impl<T: AsLeptonicaPtr> Morph for T {}
