// example marked as "not tested" because doc tests don't run in wasm32
use crate::bindgen::Main as BindgenMain;
use crate::sledgehammer::Main as SledgehammerMain;
use vanillajs::Main as JsMain;
use web_sys::{console, window, Performance};

mod bindgen;
mod sledgehammer;
mod vanillajs;

const FASTER_STYLE: &str =
    "color: #000;background-color: #00ff00;padding: 2px 4px;border-radius: 2px";
const SLOWER_STYLE: &str =
    "color: #000;background-color: #ff0000;padding: 2px 4px;border-radius: 2px";

fn main() {
    let perf = window().unwrap().performance().unwrap();

    let mut main = sledgehammer::init();

    let msg_ref: *mut SledgehammerMain = (&mut main) as *mut _;

    let mut main = bindgen::init();

    let bindgen_ref: *mut BindgenMain = (&mut main) as *mut _;

    let mut main = JsMain::new();

    let vanilla_ref: *mut JsMain = (&mut main) as *mut _;

    let vanilla_append_1000 = bench(
        &perf,
        || {
            unsafe { (*vanilla_ref).clear() };
        },
        || {
            unsafe { (*vanilla_ref).run() };
        },
    );
    unsafe { (*vanilla_ref).clear() };

    let vanilla_append_10000 = bench(
        &perf,
        || {
            unsafe { (*vanilla_ref).clear() };
        },
        || {
            unsafe { (*vanilla_ref).run_lots() };
        },
    );
    unsafe { (*vanilla_ref).clear() };

    let vanilla_add = bench(
        &perf,
        || {
            unsafe { (*vanilla_ref).clear() };
            unsafe { (*vanilla_ref).run_lots() };
        },
        || {
            unsafe { (*vanilla_ref).add() };
        },
    );
    unsafe { (*vanilla_ref).clear() };

    unsafe { (*vanilla_ref).run() };
    let vanilla_swap = bench(
        &perf,
        || {},
        || unsafe {
            (*vanilla_ref).swap_lots();
        },
    );
    unsafe { (*vanilla_ref).clear() };

    let vanilla_update = bench(
        &perf,
        || {
            unsafe { (*vanilla_ref).clear() };
            unsafe { (*vanilla_ref).run() };
        },
        || {
            unsafe { (*vanilla_ref).update() };
        },
    );
    unsafe { (*vanilla_ref).clear() };

    unsafe { (*vanilla_ref).run() };
    let vanilla_select = bench(
        &perf,
        || {},
        || {
            unsafe { (*vanilla_ref).select_lots() };
        },
    );
    unsafe { (*vanilla_ref).clear() };

    let vanilla_remove = bench(
        &perf,
        || {
            unsafe { (*vanilla_ref).run() };
        },
        || {
            unsafe { (*vanilla_ref).delete_lots() };
        },
    );
    unsafe { (*vanilla_ref).clear() };

    let sledgehammer_append_1000 = bench(
        &perf,
        || {
            unsafe { (*msg_ref).clear() };
        },
        || {
            unsafe { (*msg_ref).run() };
        },
    );
    unsafe { (*msg_ref).clear() };

    let sledgehammer_append_10000 = bench(
        &perf,
        || {
            unsafe { (*msg_ref).clear() };
        },
        || {
            unsafe { (*msg_ref).run_lots() };
        },
    );
    unsafe { (*msg_ref).clear() };

    let sledgehammer_add = bench(
        &perf,
        || {
            unsafe { (*msg_ref).clear() };
            unsafe { (*msg_ref).run_lots() };
        },
        || {
            unsafe { (*msg_ref).add() };
        },
    );
    unsafe { (*msg_ref).clear() };

    unsafe { (*msg_ref).run() };
    let sledgehammer_swap = bench(
        &perf,
        || {},
        || {
            for _ in 0..1000 {
                unsafe { (*msg_ref).swap_rows() };
            }
        },
    );
    unsafe { (*msg_ref).clear() };

    let sledgehammer_update = bench(
        &perf,
        || {
            unsafe { (*msg_ref).clear() };
            unsafe { (*msg_ref).run() };
        },
        || {
            unsafe { (*msg_ref).update() };
        },
    );
    unsafe { (*msg_ref).clear() };

    unsafe { (*msg_ref).run() };
    let sledgehammer_select = bench(
        &perf,
        || {},
        || {
            for b in (1..1001).chain(1..1001) {
                unsafe { (*msg_ref).select(b) };
            }
        },
    );
    unsafe { (*msg_ref).clear() };

    let sledgehammer_remove = bench(
        &perf,
        || {
            unsafe { (*msg_ref).clear() };
            unsafe { (*msg_ref).run() };
        },
        || {
            for b in 1..1001 {
                unsafe { (*msg_ref).delete(b) };
            }
        },
    );
    unsafe { (*msg_ref).clear() };

    let wasm_bindgen_append_1000 = bench(
        &perf,
        || {
            unsafe { (*bindgen_ref).clear() };
        },
        || {
            unsafe { (*bindgen_ref).run() };
        },
    );
    unsafe { (*bindgen_ref).clear() };

    let wasm_bindgen_append_10000 = bench(
        &perf,
        || {
            unsafe { (*bindgen_ref).clear() };
        },
        || {
            unsafe { (*bindgen_ref).run_lots() };
        },
    );
    unsafe { (*bindgen_ref).clear() };

    let wasm_bindgen_add = bench(
        &perf,
        || {
            unsafe { (*bindgen_ref).clear() };
            unsafe { (*bindgen_ref).run_lots() };
        },
        || {
            unsafe { (*bindgen_ref).add() };
        },
    );
    unsafe { (*bindgen_ref).clear() };

    unsafe { (*bindgen_ref).run() };
    let wasm_bindgen_swap = bench(
        &perf,
        || {},
        || {
            for _ in 0..1000 {
                unsafe { (*bindgen_ref).swap_rows() };
            }
        },
    );
    unsafe { (*bindgen_ref).clear() };

    let wasm_bindgen_update = bench(
        &perf,
        || {
            unsafe { (*bindgen_ref).clear() };
            unsafe { (*bindgen_ref).run() };
        },
        || {
            unsafe { (*bindgen_ref).update() };
        },
    );
    unsafe { (*bindgen_ref).clear() };

    unsafe { (*bindgen_ref).run() };
    let wasm_bindgen_select = bench(
        &perf,
        || {},
        || {
            for b in (1..1001).chain(1..1001) {
                unsafe { (*bindgen_ref).select(b) };
            }
        },
    );
    unsafe { (*bindgen_ref).clear() };

    let wasm_bindgen_remove = bench(
        &perf,
        || {
            unsafe { (*bindgen_ref).clear() };
            unsafe { (*bindgen_ref).run() };
        },
        || {
            for b in 1..1001 {
                unsafe { (*bindgen_ref).delete(b) };
            }
        },
    );
    unsafe { (*bindgen_ref).clear() };

    console::log_1(&format!("sledgehammer append 1000: {}", sledgehammer_append_1000).into());
    console::log_1(&format!("wasm-bindgen append 1000: {}", wasm_bindgen_append_1000).into());
    console::log_1(&format!("vanillajs append 1000: {}", vanilla_append_1000).into());
    compare_benches(
        wasm_bindgen_append_1000,
        "sledgehammer",
        sledgehammer_append_1000,
        "wasm-bindgen",
    );
    compare_benches(
        vanilla_append_1000,
        "sledgehammer",
        sledgehammer_append_1000,
        "vanillajs",
    );
    compare_benches(
        vanilla_append_1000,
        "wasm-bindgen",
        wasm_bindgen_append_1000,
        "vanillajs",
    );
    console::log_1(&format!("sledgehammer append 10000: {}", sledgehammer_append_10000).into());
    console::log_1(&format!("wasm-bindgen append 10000: {}", wasm_bindgen_append_10000).into());
    console::log_1(&format!("vanillajs append 10000: {}", vanilla_append_10000).into());
    compare_benches(
        wasm_bindgen_append_10000,
        "sledgehammer",
        sledgehammer_append_10000,
        "wasm-bindgen",
    );
    compare_benches(
        vanilla_append_10000,
        "sledgehammer",
        sledgehammer_append_10000,
        "vanillajs",
    );
    compare_benches(
        vanilla_append_10000,
        "wasm-bindgen",
        wasm_bindgen_append_10000,
        "vanillajs",
    );
    console::log_1(&format!("sledgehammer run lots and add: {}", sledgehammer_add).into());
    console::log_1(&format!("wasm-bindgen run lots and add: {}", wasm_bindgen_add).into());
    console::log_1(&format!("vanillajs run lots and add: {}", vanilla_add).into());
    compare_benches(
        wasm_bindgen_add,
        "sledgehammer",
        sledgehammer_add,
        "wasm-bindgen",
    );
    compare_benches(vanilla_add, "sledgehammer", sledgehammer_add, "vanillajs");
    compare_benches(vanilla_add, "wasm-bindgen", wasm_bindgen_add, "vanillajs");
    console::log_1(&format!("sledgehammer swap 1000: {}", sledgehammer_swap).into());
    console::log_1(&format!("wasm-bindgen swap 1000: {}", wasm_bindgen_swap).into());
    console::log_1(&format!("vanillajs swap 1000: {}", vanilla_swap).into());
    compare_benches(
        wasm_bindgen_swap,
        "sledgehammer",
        sledgehammer_swap,
        "wasm-bindgen",
    );
    compare_benches(vanilla_swap, "sledgehammer", sledgehammer_swap, "vanillajs");
    compare_benches(vanilla_swap, "wasm-bindgen", wasm_bindgen_swap, "vanillajs");
    console::log_1(&format!("sledgehammer update: {}", sledgehammer_update).into());
    console::log_1(&format!("wasm-bindgen update: {}", wasm_bindgen_update).into());
    console::log_1(&format!("vanillajs update: {}", vanilla_update).into());
    compare_benches(
        wasm_bindgen_update,
        "sledgehammer",
        sledgehammer_update,
        "wasm-bindgen",
    );
    compare_benches(
        vanilla_update,
        "sledgehammer",
        sledgehammer_update,
        "vanillajs",
    );
    compare_benches(
        vanilla_update,
        "wasm-bindgen",
        wasm_bindgen_update,
        "vanillajs",
    );
    console::log_1(&format!("sledgehammer select all twice: {}", sledgehammer_select).into());
    console::log_1(&format!("wasm-bindgen select all twice: {}", wasm_bindgen_select).into());
    console::log_1(&format!("vanillajs select all twice: {}", vanilla_select).into());
    compare_benches(
        wasm_bindgen_select,
        "sledgehammer",
        sledgehammer_select,
        "wasm-bindgen",
    );
    compare_benches(
        vanilla_select,
        "sledgehammer",
        sledgehammer_select,
        "vanillajs",
    );
    compare_benches(
        vanilla_select,
        "wasm-bindgen",
        wasm_bindgen_select,
        "vanillajs",
    );
    console::log_1(&format!("sledgehammer remove all: {}", sledgehammer_remove).into());
    console::log_1(&format!("wasm-bindgen remove all: {}", wasm_bindgen_remove).into());
    console::log_1(&format!("vanillajs remove all: {}", vanilla_remove).into());
    compare_benches(
        wasm_bindgen_remove,
        "sledgehammer",
        sledgehammer_remove,
        "wasm-bindgen",
    );
    compare_benches(
        vanilla_remove,
        "sledgehammer",
        sledgehammer_remove,
        "vanillajs",
    );
    compare_benches(
        vanilla_remove,
        "wasm-bindgen",
        wasm_bindgen_remove,
        "vanillajs",
    );

    // geometic mean
    let sledgehammer_mean = (sledgehammer_append_1000
        * sledgehammer_append_10000
        * sledgehammer_add
        * sledgehammer_swap
        * sledgehammer_update
        * sledgehammer_select
        * sledgehammer_remove)
        .powf(1.0 / 7.0);
    let wasm_bindgen_mean = (wasm_bindgen_append_1000
        * wasm_bindgen_append_10000
        * wasm_bindgen_add
        * wasm_bindgen_swap
        * wasm_bindgen_update
        * wasm_bindgen_select
        * wasm_bindgen_remove)
        .powf(1.0 / 7.0);
    let vanilla_mean = (vanilla_append_1000
        * vanilla_append_10000
        * vanilla_add
        * vanilla_swap
        * vanilla_update
        * vanilla_select
        * vanilla_remove)
        .powf(1.0 / 7.0);
    console::log_1(&format!("sledgehammer geometric mean: {}", sledgehammer_mean).into());
    console::log_1(&format!("wasm-bindgen geometric mean: {}", wasm_bindgen_mean).into());
    console::log_1(&format!("vanillajs geometric mean: {}", vanilla_mean).into());
}

fn compare_benches(wb: f64, name: &'static str, sl: f64, alt: &'static str) {
    let precent_time = (sl / wb) * 100.0;
    if precent_time > 100.0 {
        console::log_2(
            &format!(
                "%c{} takes {:.1}% of the time that {} takes",
                name, precent_time, alt,
            )
            .into(),
            &SLOWER_STYLE.into(),
        );
    } else {
        console::log_2(
            &format!(
                "%c{} takes {:.1}% of the time that {} takes",
                name, precent_time, alt,
            )
            .into(),
            &FASTER_STYLE.into(),
        );
    }
}

fn bench(perf: &Performance, mut setup: impl FnMut(), mut f: impl FnMut()) -> f64 {
    let mut sum = 0.0;
    const N: usize = 100;
    for _ in 0..N {
        setup();
        let start = perf.now();
        f();
        let end = perf.now();
        sum += end - start;
    }
    sum / N as f64
}
