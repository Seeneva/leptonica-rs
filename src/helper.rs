// Copyright 2021 Sergei Solodovnikov
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::ffi::CString;
use std::path::Path;

#[track_caller]
pub(crate) fn path_to_cstring(path: &Path) -> Result<CString, std::ffi::NulError> {
    if cfg!(unix) {
        use std::os::unix::ffi::OsStrExt;
        CString::new(path.as_os_str().as_bytes())
    } else {
        CString::new(path.as_os_str().to_str().expect("Can't get path"))
    }
}
