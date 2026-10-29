//! Applicator systemd para políticas io_uring (gap RestrictIOUring=).
//!
//! Consume el mismo JSON que `iouring-policy-cbpf` (#38) / iouring-lab.
//! En la box: genera y valida drop-ins. El attach real (kernel ≥ 7.0) falla fuerte.

pub mod dropin;
pub mod policy;

pub use dropin::{render_dropin, simulate_retention, validate_dropin, RetentionSim};
pub use policy::{Policy, PolicyError};
