use nativefiledialog_sys::{
    nfdresult_t_NFD_CANCEL as NFD_CANCEL, nfdresult_t_NFD_ERROR as NFD_ERROR,
    nfdresult_t_NFD_OKAY as NFD_OKAY, NFD_PickFolder,
};

use super::error::Error;
use std::ffi::{CStr, CString};

pub struct FolderPicker {
    path: Option<String>,
}

impl FolderPicker {
    pub fn new() -> Self {
        FolderPicker { path: None }
    }
    pub fn suggested_start_location(&mut self, path: &str) -> &mut Self {
        self.path = Some(path.to_owned());
        self
    }
    pub fn pick_single_folder(&self) -> Result<Option<String>, Error> {
        let mut out = std::ptr::null_mut();
        unsafe {
            let result = NFD_PickFolder(
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
