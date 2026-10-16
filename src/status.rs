use crate::crd::{Condition, WidgetStatus};

/// Derive status from desired and ready replica counts.
pub fn status_for(desired: i32, ready: i32) -> WidgetStatus {
    let available = ready >= desired && desired > 0;
    WidgetStatus {
        ready_replicas: ready,
        conditions: vec![Condition {
            r#type: "Available".into(),
            status: if available { "True" } else { "False" }.into(),
            reason: if available {
                "AllReplicasReady"
            } else {
                "Progressing"
            }
            .into(),
        }],
    }
}
