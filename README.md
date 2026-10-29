# iouring-restrict-systemd

**ES:** Applicator userspace del gap `RestrictIOUring=` en systemd: genera
drop-ins `ExecStartPre=` a partir del perfil JSON de
`iouring-policy-cbpf` (#38) / iouring-lab.

**EN:** Userspace stand-in for the missing systemd `RestrictIOUring=`
directive: emit drop-ins from the same JSON policy shape as #38.

## Medición (caja)

```bash
cargo run --bin iouring-restrict -- dropin \
  --profile fixtures/default.json \
  --bin /usr/bin/iouring-restrict | tee /tmp/dropin.conf
cargo run --bin iouring-restrict -- validate --file /tmp/dropin.conf
# validate=ok

# 50 generaciones idénticas (test):
cargo test --test dropin_lab fifty_dropin_generations_stable -- --nocapture
```

`apply --fail-without-kernel-support` y el binario `iouring-restrict-enforce`
salen con código 2 **en cualquier kernel**: el attach real de
`IORING_REGISTER_BPF_FILTER_TASK` no está implementado todavía. En el runner
`ubuntu-24.04` (kernel `7.0.0-1017-azure`) también sale 2, por esa razón y no
por falta de soporte del kernel.

## Qué está / no está medido

| Qué | Dónde |
|-----|--------|
| Parseo del perfil #38 + drop-in válido | caja |
| Self-test de retención post-exec (simulado) | caja |
| `IORING_REGISTER_BPF_FILTER_TASK` real | **no implementado** (enforce.yml queda rojo siempre) |

## CI

- `.github/workflows/ci.yml`: fmt + `clippy -D warnings` + `cargo test` + reproducción de la medición. Tiene que quedar verde.
- `.github/workflows/enforce.yml`: corre `cargo run --bin iouring-restrict-enforce` de verdad en el runner, sin `continue-on-error` ni `#[ignore]`. Hoy queda **rojo siempre** (exit 2), también en kernel ≥ 7.0, porque el attach task-level no está implementado; no es un skip.

## Fuentes leídas

- Merge Axboe 591beb0 (containers and systemd como consumidores)
- https://www.freedesktop.org/software/systemd/man/latest/systemd.directives.html (0 hits RestrictIOUring)
- man `io_uring_register_bpf_filter_task(3)`
- LWN 1054225; iouring-lab `profile/default.json`

License: Apache-2.0
