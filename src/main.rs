// example marked as "not tested" because doc tests don't run in wasm32
use crate::bindgen::Main as BindgenMain;
use crate::sledgehammer::Main as SledgehammerMain;
use easybench_wasm::{bench_env, Stats};
use web_sys::console;

mod bindgen;
mod sledgehammer;
const FASTER_STYLE: &str =
    "color: #000;background-color: #00ff00;padding: 2px 4px;border-radius: 2px";
const SLOWER_STYLE: &str =
    "color: #000;background-color: #ff0000;padding: 2px 4px;border-radius: 2px";

fn main() {
    let mut main = sledgehammer::init();

    let msg_ref: *mut SledgehammerMain = (&mut main) as *mut _;

    let mut main = bindgen::init();

    let bindgen_ref: *mut BindgenMain = (&mut main) as *mut _;

    let s = 0;

    let sledgehammer_append_1000 = bench_env((msg_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).run() };
    });
    console::log_1(&format!("sledgehammer append 1000: {}", sledgehammer_append_1000).into());
    unsafe { (*msg_ref).clear() };

    let wasm_bindgen_append_1000 = bench_env((bindgen_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).run() };
    });
    console::log_1(&format!("wasm-bindgen append 1000: {}", wasm_bindgen_append_1000).into());
    unsafe { (*bindgen_ref).clear() };

    compare_benches(wasm_bindgen_append_1000, sledgehammer_append_1000);

    let sledgehammer_append_10000 = bench_env((msg_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).run_lots() };
    });
    console::log_1(&format!("sledgehammer append 10000: {}", sledgehammer_append_10000).into());
    unsafe { (*msg_ref).clear() };

    let wasm_bindgen_append_10000 = bench_env((bindgen_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).run_lots() };
    });

    console::log_1(&format!("wasm-bindgen append 10000: {}", wasm_bindgen_append_10000).into());
    unsafe { (*bindgen_ref).clear() };

    compare_benches(wasm_bindgen_append_10000, sledgehammer_append_10000);

    let sledgehammer_run_lots_add = bench_env((msg_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).run_lots() };
        unsafe { (**r).add() };
    });
    console::log_1(
        &format!(
            "sledgehammer run lots and add: {}",
            sledgehammer_run_lots_add
        )
        .into(),
    );
    unsafe { (*msg_ref).clear() };

    let wasm_bindgen_run_lots_add = bench_env((bindgen_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).run_lots() };
        unsafe { (**r).add() };
    });
    console::log_1(
        &format!(
            "wasm-bindgen run lots and add: {}",
            wasm_bindgen_run_lots_add
        )
        .into(),
    );
    unsafe { (*bindgen_ref).clear() };

    compare_benches(wasm_bindgen_run_lots_add, sledgehammer_run_lots_add);

    unsafe { (*msg_ref).run() };
    let sledgehammer_swap = bench_env((msg_ref, s), |(r, i)| {
        *i = 1;
        for _ in 0..1000 {
            unsafe { (**r).swap_rows() };
        }
    });
    console::log_1(&format!("sledgehammer swap 1000: {}", sledgehammer_swap).into());
    unsafe { (*msg_ref).clear() };

    unsafe { (*bindgen_ref).run() };
    let wasm_bindgen_swap = bench_env((bindgen_ref, s), |(r, i)| {
        *i = 1;
        for _ in 0..1000 {
            unsafe { (**r).swap_rows() };
        }
    });
    console::log_1(&format!("wasm-bindgen swap 1000: {}", wasm_bindgen_swap).into());
    unsafe { (*bindgen_ref).clear() };

    compare_benches(wasm_bindgen_swap, sledgehammer_swap);

    unsafe { (*msg_ref).run() };
    let sledgehammer_update = bench_env((msg_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).update() };
    });
    console::log_1(&format!("sledgehammer update: {}", sledgehammer_update).into());
    unsafe { (*msg_ref).clear() };

    unsafe { (*bindgen_ref).run() };
    let wasm_bindgen_update = bench_env((bindgen_ref, s), |(r, i)| {
        *i = 1;
        unsafe { (**r).update() };
    });
    console::log_1(&format!("wasm-bindgen update: {}", wasm_bindgen_update).into());
    unsafe { (*bindgen_ref).clear() };

    compare_benches(wasm_bindgen_update, sledgehammer_update);

    unsafe { (*msg_ref).run() };
    let sledgehammer_select = bench_env((msg_ref, s), |(r, i)| {
        *i = 1;
        for b in (0..1000).chain(0..1000) {
            unsafe { (**r).select(b) };
        }
    });
    console::log_1(&format!("sledgehammer select all twice: {}", sledgehammer_select).into());
    unsafe { (*msg_ref).clear() };

    unsafe { (*bindgen_ref).run() };
    let wasm_bindgen_select = bench_env((bindgen_ref, s), |(r, i)| {
        *i = 1;
        for b in (0..1000).chain(0..1000) {
            unsafe { (**r).select(b) };
        }
    });
    console::log_1(&format!("wasm-bindgen select all twice: {}", wasm_bindgen_select).into());
    unsafe { (*bindgen_ref).clear() };

    compare_benches(wasm_bindgen_select, sledgehammer_select);

    unsafe { (*msg_ref).run() };
    let sledgehammer_remove = bench_env((msg_ref, s), |(r, i)| {
        *i = 1;
        for b in 0..1000 {
            unsafe { (**r).delete(b) };
        }
    });
    console::log_1(&format!("sledgehammer remove all: {}", sledgehammer_remove).into());
    unsafe { (*msg_ref).clear() };

    unsafe { (*bindgen_ref).run() };
    let wasm_bindgen_remove = bench_env((bindgen_ref, s), |(r, i)| {
        *i = 1;
        for b in 0..1000 {
            unsafe { (**r).delete(b) };
        }
    });
    console::log_1(&format!("wasm-bindgen remove all: {}", wasm_bindgen_remove).into());
    unsafe { (*bindgen_ref).clear() };

    compare_benches(wasm_bindgen_remove, sledgehammer_remove);
}

fn compare_benches(wb: Stats, sl: Stats) {
    if wb.ns_per_iter > sl.ns_per_iter {
        console::log_2(
            &format!(
                "%csledgehammer is {}% faster",
                100.0 - (sl.ns_per_iter / wb.ns_per_iter) * 100.0,
            )
            .into(),
            &FASTER_STYLE.into(),
        );
    } else {
        console::log_2(
            &format!(
                "%csledgehammer is {}% slower",
                (sl.ns_per_iter / wb.ns_per_iter) * 100.0 - 100.0,
            )
            .into(),
            &SLOWER_STYLE.into(),
        );
    }
}
