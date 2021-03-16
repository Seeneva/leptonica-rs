// Copyright 2021 Sergei Solodovnikov
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Docstrings for leptonica ffi functions was copied from: 
//      https://github.com/DanBloomberg/leptonica/blob/master/src/grayquant.c
//
// These docstrings license:
//      BSD 2-Clause License
//      Copyright (C) 2001 Leptonica. 
//      All rights reserved.
use std::convert::TryInto;

use super::{AsLeptonicaPtr, Pix, Result};

pub trait GrayQuant: AsLeptonicaPtr {
    //*********************************************************************************************
    // ====================== Simple (pixelwise) binarization with fixed threshold ================
    //*********************************************************************************************

    ///param   pixs     4 or 8 bpp
    ///param   thresh   threshold value
    ///return  pixd 1 bpp, or NULL on error
    ///
    ///Notes:
    ///     (1) If the source pixel is less than the threshold value,
    ///         the dest will be 1; otherwise, it will be 0.
    ///     (2) For example, for 8 bpp src pix, if %thresh == 256, the dest
    ///         1 bpp pix is all ones (fg), and if %thresh == 0, the dest
    ///         pix is all zeros (bg).
    fn threshold_to_binary(&self, thresh: i32) -> Result<Pix> {
        unsafe { leptonica_sys::pixThresholdToBinary(self.as_ptr(), thresh as _) }.try_into()
    }
}

impl<T: AsLeptonicaPtr> GrayQuant for T {}
