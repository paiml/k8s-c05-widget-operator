use kube::CustomResourceExt;
use std::io::Read;
use widget_operator::{Widget, admission, controller};

fn review_from_stdin() -> serde_json::Value {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("crd") => println!("{}", serde_json::to_string_pretty(&Widget::crd()).unwrap()),
        Some("admit") => {
            let review = review_from_stdin();
            let v = admission::validate(&review);
            println!("{}", admission::response(&review, &v));
        }
        Some("mutate") => println!(
            "{}",
            serde_json::Value::Array(admission::mutate(&review_from_stdin()))
        ),
        Some("run") => tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(controller::run())
            .unwrap(),
        _ => eprintln!("usage: widget-operator crd|admit|mutate|run"),
    }
}
