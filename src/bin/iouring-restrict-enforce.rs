//! CI enforce job: el attach task-level real todavía no está implementado.
//!
//! Sale siempre con exit 2, en cualquier kernel, para que el workflow
//! `enforce.yml` no muestre verde algo que no se ejecutó.

use std::process::ExitCode;

fn main() -> ExitCode {
    let release =
        std::fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or_else(|_| "unknown".into());
    println!("kernel={}", release.trim());
    eprintln!(
        "enforce FAIL: el attach real de IORING_REGISTER_BPF_FILTER_TASK no está \
         implementado en este crate (solo genera y valida drop-ins); \
         se niega a reportar éxito sin ejecutarlo"
    );
    ExitCode::from(2)
}
