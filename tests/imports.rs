extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                extern "JS" {
                    fn foo(s: &str);
                    fn another(a: u32) -> i32;
                    fn take_and_return_bool(a: bool) -> bool;
                }
                pub fn bar(s: &str) {
                    foo(s);
                }
                pub fn another_thunk(a: u32) -> i32 {
                    another(a)
                }
                pub fn bool_thunk(a: bool) -> bool {
                    take_and_return_bool(a)
                }
            }
        "#)
        .file("test.ts", r#"
            import { Exports, Imports } from "./out";
            import * as assert from "assert";

            let ARG: string | null = null;
            let ANOTHER_ARG: number | null = null;

            export const imports: Imports = {
                foo(s) {
                    assert.strictEqual(ARG, null);
                    assert.strictEqual(s, "foo");
                    ARG = s;
                },
                another(s) {
                    assert.strictEqual(ANOTHER_ARG, null);
                    assert.strictEqual(s, 21);
                    ANOTHER_ARG = s;
                    return 35;
                },
                take_and_return_bool(s: boolean): boolean {
                    return s;
                },
            };

            export function test(wasm: Exports) {
                assert.strictEqual(ARG, null);
                wasm.bar("foo");
                assert.strictEqual(ARG, "foo");

                assert.strictEqual(ANOTHER_ARG, null);
                assert.strictEqual(wasm.another_thunk(21), 35);
                assert.strictEqual(ANOTHER_ARG, 21);

                assert.strictEqual(wasm.bool_thunk(true), true);
                assert.strictEqual(wasm.bool_thunk(false), false);
            }
        "#)
        .test();
}
