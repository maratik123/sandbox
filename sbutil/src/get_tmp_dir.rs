use crate::consts::{dir, env_key};
use crate::SetupSandboxError;
use std::path::PathBuf;
use std::{env, fs};

pub fn get_tmp_dir() -> Result<PathBuf, SetupSandboxError> {
    fs::canonicalize(env::var_os(env_key::TMPDIR).unwrap_or(dir::TMP.into()))
        .map_err(SetupSandboxError::GetTmpDir)
}
