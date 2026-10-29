//! CI enforce job: must fail when kernel cannot apply task-level io_uring BPF.

use std::process::ExitCode;

fn main() -> ExitCode {
    let release =
        std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_else(|_| "unknown".into());
    println!("kernel={}", release.trim());
    eprintln!(
        "IORING_REGISTER_BPF_FILTER_TASK / RestrictIOUring applicator \
         requires Linux >= 7.0; refusing to skip"
    );
    ExitCode::from(2)
}
