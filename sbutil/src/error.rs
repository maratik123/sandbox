use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetupSandboxError {
    #[error("failed to get current directory: {0}")]
    GetCurrentDir(#[source] io::Error),
    #[error("failed to get tmp_dir: {0}")]
    GetTmpDir(#[source] io::Error),
}
