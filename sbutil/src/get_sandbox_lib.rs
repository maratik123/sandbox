use crate::consts::LIB_NAME;
use libloading::{library_filename, Library};
use std::path::PathBuf;

/// Always let the dynamic loader do the searching rather than hard coding the
/// full path.  This way, things like multilib, testing, local runs work easier.
///
/// Make an exception for non-standard setups (i.e. prefix) where libsandbox is
/// installed into a path that is not in ld.so.conf.
pub fn get_sandbox_lib() -> PathBuf {
    let path = PathBuf::from(library_filename(LIB_NAME));
    if let Some(lib_sandbox_path) =
        option_env!("LIBSANDBOX_PATH").filter(|s| !s.starts_with("/usr/lib"))
    {
        if unsafe { Library::new(&path) }.is_err() {
            return PathBuf::from(lib_sandbox_path).join(path);
        }
    }
    path
}
