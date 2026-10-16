use serde_json::json;
use widget_operator::WidgetSpec;
use widget_operator::admission::{mutate, response, validate};
use widget_operator::reconcile::{Action, FINALIZER, desired_deployment, needs_cleanup, plan};
use widget_operator::status::status_for;

fn spec(n: i32) -> WidgetSpec {
    WidgetSpec {
        replicas: n,
        image: "nginx:1.27".into(),
    }
}
fn review(n: serde_json::Value) -> serde_json::Value {
    json!({"request": {"uid": "u1", "object": {"spec": {"replicas": n}}}})
}

#[test]
fn plan_covers_create_scale_noop() {
    assert_eq!(plan(&spec(2), None), Action::Create);
    assert_eq!(plan(&spec(3), Some(1)), Action::Scale(3));
    assert_eq!(plan(&spec(2), Some(2)), Action::Noop);
}

#[test]
fn deployment_carries_spec() {
    let d = desired_deployment("w", &spec(4));
    assert_eq!(d["spec"]["replicas"], 4);
    assert_eq!(
        d["spec"]["template"]["spec"]["containers"][0]["image"],
        "nginx:1.27"
    );
}

#[test]
fn finalizer_only_when_deleting_and_present() {
    let f = vec![FINALIZER.to_string()];
    assert!(needs_cleanup(true, &f));
    assert!(!needs_cleanup(false, &f));
    assert!(!needs_cleanup(true, &[]));
}

#[test]
fn status_conditions() {
    assert_eq!(status_for(2, 2).conditions[0].status, "True");
    assert_eq!(status_for(2, 1).conditions[0].reason, "Progressing");
    assert_eq!(status_for(0, 0).conditions[0].status, "False");
}

#[test]
fn validate_bounds() {
    assert!(validate(&review(json!(3))).allowed);
    let bad = validate(&review(json!(9)));
    assert!(!bad.allowed && bad.message.contains("outside"));
    assert!(!validate(&review(json!(0))).allowed);
    assert!(!validate(&json!({"request": {}})).allowed);
}

#[test]
fn mutate_adds_label_both_shapes() {
    let bare = mutate(&review(json!(1)));
    assert_eq!(bare[0]["path"], "/metadata/labels");
    let with = json!({"request": {"object": {"metadata": {"labels": {"a": "b"}}}}});
    assert_eq!(mutate(&with)[0]["path"], "/metadata/labels/managed-by");
}

#[test]
fn response_echoes_uid() {
    let r = review(json!(2));
    let out = response(&r, &validate(&r));
    assert_eq!(out["response"]["uid"], "u1");
    assert_eq!(out["response"]["allowed"], true);
    assert_eq!(
        response(&json!({}), &validate(&json!({})))["response"]["uid"],
        serde_json::Value::Null
    );
}

#[test]
fn validate_iff_within_bound() {
    for n in -10..20 {
        assert_eq!(
            validate(&review(json!(n))).allowed,
            (1..=5).contains(&n),
            "n={n}"
        );
    }
}

#[test]
#[ignore]
fn calls_nothing() {
    assert_eq!(1 + 1, 2);
}
