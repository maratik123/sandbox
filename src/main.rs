use log::{debug, Level, LevelFilter};
use sandbox::args_parser;
use sandbox::dirs::TMPDIR;
use sandbox::environment::{env_key, has_env, is_env_on};
use std::path::PathBuf;
use std::{env, fs, io};
use thiserror::Error;

const SANDBOX_BANNER: &str =
    "============================= Gentoo path sandbox ==============================";

#[derive(Debug)]
struct SandboxInfo {
    work_dir: Option<PathBuf>,
    tmp_dir: PathBuf,
}

fn main() {
    let args = args_parser::parse().unwrap();
    let log_level = args
        .program_args
        .map_or(LevelFilter::Debug, |_| LevelFilter::Info);
    env_logger::builder().filter_level(log_level).init();

    debug!("{SANDBOX_BANNER}");

    if !is_env_on(env_key::SANDBOX_TESTING) && has_env(env_key::SANDBOX_ACTIVE) {
        panic!("not launching a new sandbox as one is already running in this process hierarchy");
    }

    debug!("Detection of the support files.");

    let sandbox_info = setup_sandbox(log_level >= Level::Debug).expect("failed to setup sandbox");
    println!("{sandbox_info:#?}");
}

#[derive(Error, Debug)]
enum SetupSandboxError {
    #[error("failed to get current directory: {0}")]
    GetCurrentDir(#[source] io::Error),
    #[error("failed to get tmp_dir: {0}")]
    GetTmpDir(#[source] io::Error),
}

fn setup_sandbox(interactive: bool) -> Result<SandboxInfo, SetupSandboxError> {
    let work_dir = if !has_env(env_key::PORTAGE_TMPDIR) {
        let work_dir = env::current_dir().map_err(SetupSandboxError::GetCurrentDir)?;
        if interactive {
            env::set_var(env_key::SANDBOX_WORKDIR, &work_dir)
        }
        Some(work_dir)
    } else {
        None
    };
    let tmp_dir = get_tmp_dir()?;
    Ok(SandboxInfo { work_dir, tmp_dir })
}

fn get_tmp_dir() -> Result<PathBuf, SetupSandboxError> {
    fs::canonicalize(env::var_os(env_key::TMPDIR).unwrap_or(TMPDIR.into()))
        .map_err(SetupSandboxError::GetTmpDir)
}
