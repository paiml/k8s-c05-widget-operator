use crate::crd::WidgetSpec;
use serde_json::{Value, json};

pub const FINALIZER: &str = "demo.dev/cleanup";

#[derive(Debug, PartialEq)]
pub enum Action {
    Create,
    Scale(i32),
    Noop,
}

/// Decide what to do given the spec and the observed Deployment replicas.
pub fn plan(spec: &WidgetSpec, observed: Option<i32>) -> Action {
    match observed {
        None => Action::Create,
        Some(n) if n != spec.replicas => Action::Scale(spec.replicas),
        Some(_) => Action::Noop,
    }
}

/// The child Deployment a Widget owns.
pub fn desired_deployment(name: &str, spec: &WidgetSpec) -> Value {
    json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": { "name": name, "labels": { "app": name } },
        "spec": {
            "replicas": spec.replicas,
            "selector": { "matchLabels": { "app": name } },
            "template": {
                "metadata": { "labels": { "app": name } },
                "spec": { "containers": [{ "name": "app", "image": spec.image }] }
            }
        }
    })
}

/// A deleted object with our finalizer still set needs cleanup before release.
pub fn needs_cleanup(being_deleted: bool, finalizers: &[String]) -> bool {
    being_deleted && finalizers.iter().any(|f| f == FINALIZER)
}
