use nativefiledialog_sys::{
    nfdresult_t_NFD_CANCEL as NFD_CANCEL, nfdresult_t_NFD_ERROR as NFD_ERROR,
    nfdresult_t_NFD_OKAY as NFD_OKAY, NFD_SaveDialog,
};

use std::ffi::{CStr, CString};

use super::error::Error;

pub struct FileSavePicker {
    filter: Option<String>,
    path: Option<String>,
}

impl FileSavePicker {
    pub fn new() -> Self {
        FileSavePicker {
            filter: None,
            path: None,
        }
    }
    pub fn file_type_filter<'a, I, S>(&mut self, list: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for item in list {
            let item = item.as_ref();
            self.filter = match &self.filter {
                Some(filter) => Some(format!("{}{}", filter, item)),
                None => Some(item.to_owned()),
            }
        }
        self
    }
    pub fn suggested_start_location(&mut self, path: &str) -> &mut Self {
        self.path = Some(path.to_owned());
        self
    }

    pub fn pick_save_file(&self) -> Result<Option<String>, Error> {
        let mut out = std::ptr::null_mut();
        unsafe {
            let result = NFD_SaveDialog(
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
}
