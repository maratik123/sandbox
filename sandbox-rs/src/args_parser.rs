use anyhow::Result;
use clap::{Args, Parser};
use enumflags2::{bitflags, BitFlags};
use nix::unistd::{access, AccessFlags};
use std::ffi::OsString;
use std::path::PathBuf;

#[bitflags]
#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NS {
    CGroup,
    IPC,
    Mnt,
    Net,
    Pid,
    SysV,
    Time,
    User,
    UTS,
}

#[derive(Debug, Clone)]
pub struct SbRunOptions {
    pub program_args: Option<ProgramArgs>,
    pub ns_opts: Option<BitFlags<NS>>,
    pub run_bash: bool,
}

#[derive(Debug, Clone)]
pub struct ProgramArgs {
    pub program: PathBuf,
    pub args: Vec<OsString>,
}

/// Sandbox will start up a sandbox session and execute the specified program.
/// If no program is specified, an interactive shell is automatically launched.
/// You can use this to quickly test out sandbox behavior.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct SbArgs {
    /// Enable  the use of namespaces
    #[arg(long)]
    ns_on: bool,
    /// Disable the use of namespaces
    #[arg(long)]
    ns_off: bool,
    /// Enable  the use of cgroup namespaces
    #[arg(long)]
    ns_cgroup_on: bool,
    /// Disable the use of cgroup namespaces
    #[arg(long)]
    ns_cgroup_off: bool,
    /// Enable  the use of IPC (and System V) namespaces
    #[arg(long)]
    ns_ipc_on: bool,
    /// Disable the use of IPC (and System V) namespaces
    #[arg(long)]
    ns_ipc_off: bool,
    /// Enable  the use of mount namespaces
    #[arg(long)]
    ns_mnt_on: bool,
    /// Disable the use of mount namespaces
    #[arg(long)]
    ns_mnt_off: bool,
    /// Enable  the use of network namespaces
    #[arg(long)]
    ns_net_on: bool,
    /// Disable the use of network namespaces
    #[arg(long)]
    ns_net_off: bool,
    /// Enable  the use of process (pid) namespaces
    #[arg(long)]
    ns_pid_on: bool,
    /// Disable the use of process (pid) namespaces
    #[arg(long)]
    ns_pid_off: bool,
    /// Enable  the use of System V namespaces
    #[arg(long)]
    ns_sysv_on: bool,
    /// Disable the use of System V namespaces
    #[arg(long)]
    ns_sysv_off: bool,
    /// Enable  the use of time namespaces
    #[arg(long)]
    ns_time_on: bool,
    /// Disable the use of time namespaces
    #[arg(long)]
    ns_time_off: bool,
    /// Enable  the use of user namespaces
    #[arg(long)]
    ns_user_on: bool,
    /// Disable the use of user namespaces
    #[arg(long)]
    ns_user_off: bool,
    /// Enable  the use of UTS (hostname/uname) namespaces
    #[arg(long)]
    ns_uts_on: bool,
    /// Disable the use of UTS (hostname/uname) namespaces
    #[arg(long)]
    ns_uts_off: bool,
    /// Run command through bash shell
    #[arg(short = 'c', long = "bash")]
    run_bash: bool,
    #[command(flatten)]
    program_args: ProgramWithArgs,
}

#[derive(Args, Debug, Clone)]
struct ProgramWithArgs {
    /// Program to start
    program: Option<PathBuf>,
    /// Program arguments
    args: Vec<OsString>,
}

pub fn parse() -> Result<SbRunOptions> {
    let args = SbArgs::parse();
    let ns = !args.ns_off && args.ns_on;
    let run_bash = args.run_bash
        || if let Some(program) = &args.program_args.program {
            access(program, AccessFlags::X_OK).is_err()
        } else {
            false
        };
    let program_args = args.program_args.program.map(|program| ProgramArgs {
        program,
        args: args.program_args.args,
    });
    let ns_opts = if ns {
        Some(
            [
                bf(args.ns_cgroup_off, args.ns_cgroup_on, NS::CGroup),
                bf(args.ns_ipc_off, args.ns_ipc_on, NS::IPC),
                bf(args.ns_mnt_off, args.ns_mnt_on, NS::Mnt),
                bf(args.ns_net_off, args.ns_net_on, NS::Net),
                bf(args.ns_pid_off, args.ns_pid_on, NS::Pid),
                bf(args.ns_sysv_off, args.ns_sysv_on, NS::SysV),
                bf(args.ns_time_off, args.ns_time_on, NS::Time),
                bf(args.ns_user_off, args.ns_user_on, NS::User),
                bf(args.ns_uts_off, args.ns_uts_on, NS::UTS),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    } else {
        None
    };
    Ok(SbRunOptions {
        run_bash,
        program_args,
        ns_opts,
    })
}

#[inline]
fn bf(off: bool, on: bool, flag: NS) -> Option<NS> {
    if !off && on {
        Some(flag)
    } else {
        None
    }
}
