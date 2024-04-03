//! The example from the readme!
//!
//! This example demonstrates how to create a simple counter app with dioxus. The `Signal` type wraps inner values,
//! making them `Copy`, allowing them to be freely used in closures and async functions. `Signal` also provides
//! helper methods like AddAssign, SubAssign, toggle, etc, to make it easy to update the value without running
//! into lock issues.

use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut data = use_signal(|| false);

    if data() {
        println!("RAN1");
        rsx!("true")
    } else {
        println!("RAN2");
        data.set(true);
        rsx!("false")
    }
}
