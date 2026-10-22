# widget-operator

A small Kubernetes operator in Rust with kube-rs: a `Widget` CRD, a reconcile loop,
status conditions, a finalizer, and validating and mutating admission logic.

Every decision is a pure function (`reconcile`, `status`, `admission`) covered at 100%
by real execution tracing; `controller.rs` and `main.rs` only do I/O.

    cargo run -q -- crd | kubectl apply -f -
    cargo run -q -- run
    bash scripts/coverage.sh && bash scripts/falsify.sh

## Installation

```sh
git clone https://github.com/paiml/k8s-c05-widget-operator
cd k8s-c05-widget-operator
cargo build --release
```

Requires a Rust toolchain and, to run against a cluster, `kubectl` and a kind cluster.

## Usage

```sh
cargo run -q -- crd | kubectl apply -f -   # install the Widget CRD
cargo run -- run                     # start the operator
bash scripts/coverage.sh             # 100% line and function coverage gate
```

## Contributing

Open a pull request against `main`. Every decision lives in a pure function with a test; `scripts/coverage.sh` must stay at 100% for the measured modules.

## License

MIT. See `LICENSE`.
