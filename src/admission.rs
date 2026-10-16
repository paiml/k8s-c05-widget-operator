use serde_json::{Value, json};

pub const MAX_REPLICAS: i64 = 5;

#[derive(Debug, PartialEq)]
pub struct Verdict {
    pub allowed: bool,
    pub message: String,
}

fn replicas(review: &Value) -> Option<i64> {
    review.pointer("/request/object/spec/replicas")?.as_i64()
}

/// Validating webhook: refuse widgets outside 1..=MAX_REPLICAS.
pub fn validate(review: &Value) -> Verdict {
    match replicas(review) {
        Some(n) if (1..=MAX_REPLICAS).contains(&n) => Verdict {
            allowed: true,
            message: String::new(),
        },
        Some(n) => Verdict {
            allowed: false,
            message: format!("replicas {n} outside 1..={MAX_REPLICAS}"),
        },
        None => Verdict {
            allowed: false,
            message: "spec.replicas is required".into(),
        },
    }
}

/// Mutating webhook: JSON Patch that adds a managed-by label when missing.
pub fn mutate(review: &Value) -> Vec<Value> {
    let has = review.pointer("/request/object/metadata/labels").is_some();
    if has {
        vec![
            json!({"op": "add", "path": "/metadata/labels/managed-by", "value": "widget-operator"}),
        ]
    } else {
        vec![
            json!({"op": "add", "path": "/metadata/labels", "value": {"managed-by": "widget-operator"}}),
        ]
    }
}

/// Wrap a verdict in an AdmissionReview response.
pub fn response(review: &Value, v: &Verdict) -> Value {
    json!({
        "apiVersion": "admission.k8s.io/v1",
        "kind": "AdmissionReview",
        "response": {
            "uid": review.pointer("/request/uid").cloned().unwrap_or(Value::Null),
            "allowed": v.allowed,
            "status": { "message": v.message }
        }
    })
}
