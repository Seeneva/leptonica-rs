// Copyright 2021 Sergei Solodovnikov
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Docstrings for leptonica ffi functions was copied from: 
//      https://github.com/DanBloomberg/leptonica/blob/master/src/graymorph.c
//
// These docstrings license:
//      BSD 2-Clause License
//      Copyright (C) 2001 Leptonica. 
//      All rights reserved.
use std::convert::TryInto;

use super::{AsLeptonicaPtr, Pix, Result};

pub trait GrayMorph: AsLeptonicaPtr {
    fn erode_gray(&self, h_size: i32, v_size: i32) -> Result<Pix> {
        unsafe { leptonica_sys::pixErodeGray(self.as_ptr(), h_size as _, v_size as _) }.try_into()
    }

    fn dilate_gray(&self, h_size: i32, v_size: i32) -> Result<Pix> {
        unsafe { leptonica_sys::pixDilateGray(self.as_ptr(), h_size as _, v_size as _) }.try_into()
    }
}

impl<T: AsLeptonicaPtr> GrayMorph for T {}
