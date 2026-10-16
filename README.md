# widget-operator

A small Kubernetes operator in Rust with kube-rs: a `Widget` CRD, a reconcile loop,
status conditions, a finalizer, and validating and mutating admission logic.

Every decision is a pure function (`reconcile`, `status`, `admission`) covered at 100%
by real execution tracing; `controller.rs` and `main.rs` only do I/O.

    cargo run -q -- crd | kubectl apply -f -
    cargo run -q -- run
    bash scripts/coverage.sh && bash scripts/falsify.sh
