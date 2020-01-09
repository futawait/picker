use nativefiledialog_sys::{
    nfdpathset_t, nfdresult_t_NFD_CANCEL as NFD_CANCEL, nfdresult_t_NFD_ERROR as NFD_ERROR,
    nfdresult_t_NFD_OKAY as NFD_OKAY, NFD_OpenDialog, NFD_OpenDialogMultiple, NFD_PathSet_Free,
    NFD_PathSet_GetCount, NFD_PathSet_GetPath,
};

use super::error::Error;
use std::ffi::{CStr, CString};

pub struct FileOpenPicker {
    filter: Option<String>,
    path: Option<String>,
}

impl FileOpenPicker {
    pub fn new() -> Self {
        FileOpenPicker {
            filter: None,
            path: None,
        }
    }
    pub fn file_type_filter<'a, I, S>(&mut self, list: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for (n, item) in list.into_iter().enumerate() {
            let item = item.as_ref();
            self.filter = match &self.filter {
                Some(filter) => {
                    if n == 0 {
                        Some(format!("{};{}", filter, item))
                    } else {
                        Some(format!("{},{}", filter, item))
                    }
                }
                None => Some(item.to_owned()),
            }
        }
        self
    }
    pub fn suggested_start_location(&mut self, path: &str) -> &mut Self {
        self.path = Some(path.to_owned());
        self
    }

    pub fn pick_single_file(&self) -> Result<Option<String>, Error> {
        let mut out = std::ptr::null_mut();
        unsafe {
            let result = NFD_OpenDialog(
                match self.filter.clone() {
                    Some(filter) => CString::new(filter)?.into_raw(),
                    None => std::ptr::null(),
                },
                match self.path.clone() {
                    Some(path) => CString::new(path)?.into_raw(),
                    None => std::ptr::null(),
                },
                &mut out,
            );
            match result {
                NFD_ERROR => Err(Error::IO),
                NFD_OKAY => Ok(Some(CStr::from_ptr(out).to_string_lossy().into_owned())),
                NFD_CANCEL => Ok(None),
                _ => Err(Error::C),
            }
        }
    }

    pub fn pick_multiple_files(&self) -> Result<Option<Vec<String>>, Error> {
        let mut out = nfdpathset_t {
            buf: std::ptr::null_mut(),
            indices: std::ptr::null_mut(),
            count: 0,
        };
        unsafe {
            let result = NFD_OpenDialogMultiple(
                match self.filter.clone() {
                    Some(filter) => CString::new(filter)?.into_raw(),
                    None => std::ptr::null(),
                },
                match self.path.clone() {
                    Some(path) => CString::new(path)?.into_raw(),
                    None => std::ptr::null(),
                },
                &mut out,
            );
            match result {
                NFD_ERROR => Err(Error::IO),
                NFD_OKAY => {
                    let mut path_set = Vec::new();
                    let count = NFD_PathSet_GetCount(&out);
                    for n in 0..count {
                        let path_ptr = NFD_PathSet_GetPath(&out, n);
                        let path = CStr::from_ptr(path_ptr).to_string_lossy().into_owned();
                        path_set.push(path);
                    }
                    NFD_PathSet_Free(&mut out);
                    Ok(Some(path_set))
                }
                NFD_CANCEL => Ok(None),
                _ => Err(Error::C),
            }
        }
    }
}
