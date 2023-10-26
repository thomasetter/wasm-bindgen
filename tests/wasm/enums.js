const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_c_style_enum = () => {
    assert.strictEqual(wasm.Color_Tags.Green, 0);
    assert.strictEqual(wasm.Color_Tags.Yellow, 1);
    assert.strictEqual(wasm.Color_Tags.Red, 2);
    assert.strictEqual(wasm.Color_Tags[0], 'Green');
    assert.strictEqual(wasm.Color_Tags[1], 'Yellow');
    assert.strictEqual(wasm.Color_Tags[2], 'Red');
    assert.strictEqual(Object.keys(wasm.Color_Tags).length, 6);

    assert.strictEqual(wasm.Color.Green.tag, 0);
    assert.strictEqual(wasm.Color.Yellow.tag, 1);
    assert.strictEqual(wasm.Color.Red.tag, 2);

    assert.strictEqual(wasm.enum_cycle(wasm.Color.Green), wasm.Color.Yellow);
    assert.throws(() => {wasm.enum_cycle(undefined);});
};

exports.js_c_style_enum_with_custom_values = () => {
    assert.strictEqual(wasm.ColorWithCustomValues_Tags.Green, 21);
    assert.strictEqual(wasm.ColorWithCustomValues_Tags.Yellow, 34);
    assert.strictEqual(wasm.ColorWithCustomValues_Tags.Red, 2);
    assert.strictEqual(wasm.ColorWithCustomValues_Tags[21], 'Green');
    assert.strictEqual(wasm.ColorWithCustomValues_Tags[34], 'Yellow');
    assert.strictEqual(wasm.ColorWithCustomValues_Tags[2], 'Red');
    assert.strictEqual(Object.keys(wasm.ColorWithCustomValues_Tags).length, 6);

    assert.strictEqual(wasm.enum_with_custom_values_cycle(wasm.ColorWithCustomValues.Green), wasm.ColorWithCustomValues.Yellow);
};

exports.js_handle_optional_enums = x => wasm.handle_optional_enums(x);

exports.js_expect_enum = (a, b) => {
  assert.strictEqual(a, b);
};

exports.js_expect_enum_none = a => {
  assert.strictEqual(a, undefined);
};

exports.js_renamed_enum = b => {
  assert.strictEqual(wasm.JsRenamedEnum.B, b);
};

exports.js_enum_with_error_variant = () => {
    assert.strictEqual(wasm.EnumWithErrorVariant_Tags.Error, 2);
};

exports.js_enum_type_safety = () => {
    assert.doesNotThrow(() => {wasm.enum_cycle(wasm.Color.Green);});
    // ColorCopy has the same structure and tag values, but is a different class, make sure we catch mix-ups.
    assert.throws(() => {wasm.enum_cycle(wasm.ColorCopy.Green);});
}