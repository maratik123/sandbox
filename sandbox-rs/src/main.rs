use log::{debug, Level, LevelFilter};
use sandbox_rs::args_parser;
use sandbox_rs::environment::{has_env, is_env_on};
use sbutil::consts::env_key;
use sbutil::{get_sandbox_lib, get_tmp_dir, SetupSandboxError};
use std::env;
use std::io::Write;
use std::path::PathBuf;

const SANDBOX_BANNER: &str =
    "============================= Gentoo path sandbox ==============================";

#[derive(Debug)]
struct SandboxInfo {
    work_dir: Option<PathBuf>,
    tmp_dir: PathBuf,
    home_dir: PathBuf,
    sandbox_lib: PathBuf,
}

fn main() {
    let args = args_parser::parse().unwrap();
    let log_level = args
        .program_args
        .map_or(LevelFilter::Debug, |_| LevelFilter::Info);
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(log_level)
        .init();

    debug!("{SANDBOX_BANNER}");

    if !is_env_on(env_key::SANDBOX_TESTING) && has_env(env_key::SANDBOX_ACTIVE) {
        panic!("not launching a new sandbox as one is already running in this process hierarchy");
    }

    debug!("Detection of the support files.");

    let sandbox_info = setup_sandbox(log_level >= Level::Debug).expect("failed to setup sandbox");
    println!("{sandbox_info:#?}");
}

fn setup_sandbox(interactive: bool) -> Result<SandboxInfo, SetupSandboxError> {
    let work_dir = if !has_env(env_key::PORTAGE_TMPDIR) {
        let work_dir = env::current_dir().map_err(SetupSandboxError::GetCurrentDir)?;
        if interactive {
            env::set_var(env_key::SANDBOX_WORKDIR, &work_dir)
        }
        Some(work_dir)
    } else {
        /* Portage handle setting SANDBOX_WRITE itself. */
        None
    };

    let tmp_dir = get_tmp_dir()?;
    env::set_var(env_key::TMPDIR, &tmp_dir);

    let home_dir = env::var_os(env_key::HOME)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let home_dir = tmp_dir.clone();
            env::set_var(env_key::HOME, &home_dir);
            home_dir
        });

    let sandbox_lib = get_sandbox_lib();

    Ok(SandboxInfo {
        work_dir,
        tmp_dir,
        home_dir,
        sandbox_lib,
    })
}
