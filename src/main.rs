use clap::{Parser, Subcommand};
use iouring_restrict_systemd::{
    render_dropin, simulate_retention, validate_dropin, Policy,
};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "iouring-restrict",
    about = "Genera drop-ins systemd RestrictIOUring=-like desde un perfil io_uring (#38)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Genera un drop-in .conf
    Dropin {
        #[arg(long)]
        profile: PathBuf,
        #[arg(long, default_value = "/usr/bin/iouring-restrict")]
        bin: PathBuf,
        #[arg(long, default_value = "iouring-restrict generated")]
        comment: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Valida un drop-in existente
    Validate {
        #[arg(long)]
        file: PathBuf,
    },
    /// Aplica el perfil (en box: dry-run; con --fail-without-kernel-support sale ≠0)
    Apply {
        #[arg(long)]
        profile: PathBuf,
        #[arg(long, default_value_t = false)]
        fail_without_kernel_support: bool,
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Self-test de retención post-exec (simulado en userspace)
    RetentionSelftest {
        #[arg(long)]
        profile: PathBuf,
        #[arg(long, default_value_t = true)]
        survive_exec: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Dropin {
            profile,
            bin,
            comment,
            out,
        } => {
            if let Err(e) = Policy::parse_json(&fs::read_to_string(&profile).unwrap_or_default()) {
                eprintln!("profile: {e}");
                return ExitCode::from(1);
            }
            let text = render_dropin(&profile, &bin, &comment);
            if let Err(e) = validate_dropin(&text) {
                eprintln!("internal validate: {e}");
                return ExitCode::from(1);
            }
            match out {
                Some(p) => {
                    if let Err(e) = fs::write(&p, &text) {
                        eprintln!("write {p:?}: {e}");
                        return ExitCode::from(1);
                    }
                    println!("wrote {}", p.display());
                }
                None => print!("{text}"),
            }
            ExitCode::SUCCESS
        }
        Cmd::Validate { file } => {
            let text = match fs::read_to_string(&file) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            match validate_dropin(&text) {
                Ok(()) => {
                    println!("validate=ok");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("validate=FAIL: {e}");
                    ExitCode::from(1)
                }
            }
        }
        Cmd::Apply {
            profile,
            fail_without_kernel_support,
            dry_run,
        } => {
            let raw = match fs::read_to_string(&profile) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let p = match Policy::parse_json(&raw) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            println!("profile={}", p.summary_line());
            if dry_run || !fail_without_kernel_support {
                println!("apply=dry-run (no IORING_REGISTER_BPF_FILTER on this path)");
                return ExitCode::SUCCESS;
            }
            // Fail hard: this box / typical runners lack Linux ≥ 7.0 filter register.
            let release = fs::read_to_string("/proc/sys/kernel/osrelease")
                .unwrap_or_else(|_| "unknown".into());
            eprintln!(
                "apply=FAIL: kernel {} lacks IORING_REGISTER_BPF_FILTER_TASK; \
                 refusing to skip (use VM/CI ≥ 7.0)",
                release.trim()
            );
            ExitCode::from(2)
        }
        Cmd::RetentionSelftest {
            profile,
            survive_exec,
        } => {
            let p = match Policy::parse_json(&fs::read_to_string(&profile).unwrap_or_default()) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let s = simulate_retention(&p, survive_exec);
            println!(
                "before={} after={} survive_exec={survive_exec}",
                s.restricted_before_exec, s.still_restricted_after_exec
            );
            let ok = s.still_restricted_after_exec == survive_exec;
            if ok {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
    }
}
