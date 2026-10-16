use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "demo.dev",
    version = "v1",
    kind = "Widget",
    namespaced,
    status = "WidgetStatus"
)]
pub struct WidgetSpec {
    pub replicas: i32,
    pub image: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, PartialEq)]
pub struct WidgetStatus {
    pub ready_replicas: i32,
    pub conditions: Vec<Condition>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq)]
pub struct Condition {
    pub r#type: String,
    pub status: String,
    pub reason: String,
}
