//! I/O wiring only: every decision is made by the pure functions in `reconcile` and `status`.
use crate::Widget;
use crate::reconcile::{Action, FINALIZER, desired_deployment, needs_cleanup, plan};
use crate::status::status_for;
use futures::StreamExt;
use k8s_openapi::api::apps::v1::Deployment;
use kube::Client;
use kube::api::{Api, Patch, PatchParams, ResourceExt};
use kube::runtime::{Controller, controller::Action as Next, watcher};
use serde_json::json;
use std::{sync::Arc, time::Duration};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kube: {0}")]
    Kube(#[from] kube::Error),
}

async fn reconcile(w: Arc<Widget>, client: Arc<Client>) -> Result<Next, Error> {
    let ns = w.namespace().unwrap_or_default();
    let name = w.name_any();
    let widgets: Api<Widget> = Api::namespaced((*client).clone(), &ns);
    let deploys: Api<Deployment> = Api::namespaced((*client).clone(), &ns);
    let finalizers = w.finalizers().to_vec();

    if needs_cleanup(w.metadata.deletion_timestamp.is_some(), &finalizers) {
        println!("cleanup {name}");
        let rest: Vec<_> = finalizers.into_iter().filter(|f| f != FINALIZER).collect();
        let p = json!({"metadata": {"finalizers": rest}});
        widgets
            .patch(&name, &PatchParams::default(), &Patch::Merge(&p))
            .await?;
        return Ok(Next::await_change());
    }
    if !finalizers.iter().any(|f| f == FINALIZER) {
        let mut all = finalizers;
        all.push(FINALIZER.to_string());
        let p = json!({"metadata": {"finalizers": all}});
        widgets
            .patch(&name, &PatchParams::default(), &Patch::Merge(&p))
            .await?;
    }

    let observed = deploys
        .get_opt(&name)
        .await?
        .and_then(|d| d.spec.and_then(|s| s.replicas));
    let action = plan(&w.spec, observed);
    println!("reconcile {name}: {action:?}");
    if action != Action::Noop {
        let mut d = desired_deployment(&name, &w.spec);
        d["metadata"]["ownerReferences"] = json!([{
            "apiVersion": "demo.dev/v1", "kind": "Widget", "name": name,
            "uid": w.uid().unwrap_or_default(), "controller": true, "blockOwnerDeletion": true
        }]);
        deploys
            .patch(
                &name,
                &PatchParams::apply("widget-operator"),
                &Patch::Apply(&d),
            )
            .await?;
    }
    let ready = deploys
        .get_opt(&name)
        .await?
        .and_then(|d| d.status.and_then(|s| s.ready_replicas))
        .unwrap_or(0);
    let st = json!({"status": status_for(w.spec.replicas, ready)});
    widgets
        .patch_status(&name, &PatchParams::default(), &Patch::Merge(&st))
        .await?;
    Ok(Next::requeue(Duration::from_secs(5)))
}

fn on_error(_w: Arc<Widget>, e: &Error, _c: Arc<Client>) -> Next {
    eprintln!("error: {e}");
    Next::requeue(Duration::from_secs(5))
}

pub async fn run() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let widgets: Api<Widget> = Api::all(client.clone());
    Controller::new(widgets, watcher::Config::default())
        .run(reconcile, on_error, Arc::new(client))
        .for_each(|_| async {})
        .await;
    Ok(())
}
