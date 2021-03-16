// Copyright 2021 Sergei Solodovnikov
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Docstrings for leptonica ffi functions was copied from: 
//      https://github.com/DanBloomberg/leptonica/blob/master/src/colorspace.c
//
// These docstrings license:
//      BSD 2-Clause License
//      Copyright (C) 2001 Leptonica. 
//      All rights reserved.
use super::{AsLeptonicaPtr, Pix, Result};

use std::convert::TryInto;

pub trait ColorSpace: AsLeptonicaPtr {
    //*********************************************************************************************
    // ====================== Colorspace conversion between RGB and HSV ============================
    //*********************************************************************************************

    /// Input: pixs (32 bpp RGB or 8 bpp with colormap) Return: pixd (8 bpp sat of HSV), or null on error
    /// Notes: (1) The conversion to HSV sat is in-lined here.
    ///         (2) If there is a colormap, it is removed.
    ///         (3) If you just want the saturation component, this does it at about 12 Mpixels/sec/GHz.
    fn convert_rgb_to_saturation(&self) -> Result<Pix> {
        unsafe { leptonica_sys::pixConvertRGBToSaturation(self.as_ptr()) }.try_into()
    }

    /// Input: pixs (32 bpp RGB or 8 bpp with colormap) Return: pixd (8 bpp max component intensity of HSV), or null on error
    /// Notes: (1) The conversion to HSV sat is in-lined here.
    ///         (2) If there is a colormap, it is removed.
    ///         (3) If you just want the value component, this does it at about 35 Mpixels/sec/GHz.
    fn convert_rgb_to_value(&mut self) -> Result<Pix> {
        unsafe { leptonica_sys::pixConvertRGBToValue(self.as_ptr()) }.try_into()
    }
}

impl<T: AsLeptonicaPtr> ColorSpace for T {}
