import * as wbg from "../pkg/typescript_tests";
import { expect, jest, test } from "@jest/globals";

// TODO: implement solution in
// https://www.typescriptlang.org/play?#code/KYOwrgtgBAolDeAoKKoEEoF4oEYA0yqAQllAMwEC+iiA9LVAGZggDGALgJYD2IT33ABQBDAFxRhIAJ4BKBHQaoonRlBHKQAZ3aTWwbqphyk9JWda9N3ADbAAdNe4BzQQHIAysIDusca7wSMgDcNKZK1KYRDIwCgjB2aMGIFlo29o4uRiGIwgBG2gBOwhxQrNbCmppQAGICOAD6RBXA8koADgWcAG7C7C31YJrAAHLcEJwgwtbiLAAmwIwTwLNBUKYd3b0ti8DWs1Ds3FCgmmAFLSDc7AAWE07H1kPKVYVgHGBT1lKlY229nLlbHZCCg2mBAZxWBJ8uwiiVzsJZrwvgdhE5xLVuA0ACpozQhJTrApXYAcZY-LSwt6HAqCMEQqEIpEgFE6dE1Or1XFOTTGEFrBjUajJSzsY7gaCYnF41qoACqwwAIjBqgBJYYwRUEJQYbD4fkkbAUfkAYVIABZEML2FI2i0paQpfUMAAfDlYxpQN1Ok3ZMoVKpOjDAAAefRAs0DnKaTyQ7XB1khUCZyO+bMdnO5mgS2SUAG13FIILkbHZrhVVZTdMAALqCbi5ABW4kksnEJbSkllZmUqnrTY02mrBndDRjwD5PZ753YZz4DcbdnTmGwTqzOf54U3qBnc6YUyGBNQQpo-sqo89ofDkYv4+7UHpicZwERqdR91XmbxdiIR5QBaLEtrDLCsqzYWt+2bKAWAAa0uLwQBkdsBFsLs4ynFQ1AXQcdHAkcnXHScpyUXcCnnJslzRLBPw9ddf23FBqGI0i+EYA9gD-KAT2Scpzx9RpmmOMNQBvAjBPQ0EEyTFMWTTKiaOlHk7F9fkAOLUty00Ssh3AusFxmEA4O4BCkKgDtUL4CSzBYqBsImHS9HwzkTQEw9+W4s8ow9M0rxEryGhcu8rJSV4OG4WlHyTHppigCVcmAAoiJ7U47VpGR3KtU9RVHUgrLQcQAHkm1Jdg7EYc5gAAL2AQQQGAHwgxkbViEK4qODKirqtq+rbya01BGi8Q4oS0yfQQZNgFnMjYp6n0BqmYIuOalBTBNVrGxKjrgCqmq6oa5zXInKhskmCBgE0P49ByqzQzacKxRtO10AzD00E4277oOW0WkNW93pDO6Cge76oDNBT6hU4VTC6BKVDTcsHuuFo2ScckWB4PhZk4TRWE6cZJi4XgoC8cKYM0RBmDYQm+B0GDzvqGIhEYDE6iSqBNC8Th2FYa41EYSinDZpRWEEtdvzWhjrMmvd+eizilHmNiwGsdhRElkjpemgAGXNj0yxBBEEORMAAPnvEKxWEF6cA3YXLDSBxnBEAWkjt1IgQyERXdQEKHc9q37NwxzVDEoZvZQX2PadgOwOD0cEnDikrCjlxXFCqapn8ca2RZ2jvwwSgcOHEO6gTzjI-SaOi7wkusR-ROLdKUg9ovE1BBwLWG-tlPBFYOxoq793K5cKFA+L29mkH5Ph7cUf-IhrPR9j-Ra4CxPTAK2TbLq2zVBuC4rwOEmoETOqqhFvgSYKGCzOafYia8BHiZacsYY0JOxSlAIOgbPIUQtgo1JwpMBYBwDGVRhDnCgBAcKLROAQEBrhRGXYx4135KYCujsR7VzjlKZSU8xSsAZsSCA9QkRigUspdundy7dxnq4Ih5UxhkKuM8Vui9iHMPITglerckg0CtDII22RTDYiOKhaaMCoF5G4GARGLRyggCcB8VGAQyiQhvjIt+EwoAACIYAhmEAg2wmhdG2QKHogA6gjNOUBhj1V0cCDe+8CicyGAEVGYohxA3JLkb45wYFdDuAcJGTwLAQDOiAdgECIwhJaFfPYbCpCyIsW0coUgnDEjmMCIAA
export class Foo1_Base {
  tag: Foo1_Tags;
}

const enum Foo1_Tags {
  A = 1,
  B = 3
}

// const Foo1_Tags  = {
//   A : 1,
//   "1": "A",
//   B : 3,
//   "3":"B"
// };

// class Foo1_Tags { 
//   public static readonly A = 1;
//   public static readonly B = 3; 
// }


// class Foo1 {
//   static readonly A: Foo1_A;
//   static readonly B: Foo1_B;
// }

type Foo1 = Foo1_A | Foo1_B;

class Foo1_A extends Foo1_Base {
  tag: Foo1_Tags.A
}

class Foo1_B extends Foo1_Base {
  tag: Foo1_Tags.B
}

const Foo1 = {
  A: new Foo1_A,
  B: new Foo1_B,
}

namespace Foo1 {
  type A = Foo1_A;
}

test("Foo1", () => {
  const a: typeof Foo1.A = Foo1.A;
  expect(a.tag).toStrictEqual(Foo1_Tags.A);

});

test("construction", () => {
  const a1: wbg.Foo = wbg.Foo.A;
  const a2: wbg.Foo.A = wbg.Foo.A;
  expect(a1).toStrictEqual(a2);

  //TODO: this should be a type error
  const a3: wbg.Foo.A = 1;
  expect(a1).not.toStrictEqual(a3);

  const b1: wbg.Foo = wbg.Foo.B;
  const b2: wbg.Foo.B = wbg.Foo.B;
  expect(b1).toStrictEqual(b2);

  //TODO: this should be a type error
  const b3: wbg.Foo.B = 3;
  expect(b1).not.toStrictEqual(b3);
  expect(a1).not.toStrictEqual(b1);
});

test("function calls", () => {
  const fn_expects_enum: (_: wbg.Foo) => void = wbg.fn_expects_enum;
  const fn_returns_enum: () => wbg.Foo = wbg.fn_returns_enum;
  const fn_expects_option_enum: (_?: wbg.Foo) => void = wbg.fn_expects_option_enum;
  const fn_returns_option_enum: () => wbg.Foo | undefined = wbg.fn_returns_option_enum;
  const fn_echo_option_enum: (_: wbg.Foo) => wbg.Foo | undefined = wbg.fn_echo_option_enum;

  fn_expects_enum(wbg.Foo.B);
  expect(() => { fn_expects_enum(undefined); }).toThrow('expected instance of Foo_Base');
  fn_expects_option_enum(wbg.Foo.B);
  // TODO: should be a type error
  fn_expects_option_enum(3);
  fn_expects_option_enum(undefined);
  expect(fn_returns_enum()).toStrictEqual(wbg.Foo.A);
  expect(fn_returns_option_enum()).toStrictEqual(wbg.Foo.A);

  expect(fn_echo_option_enum(undefined)).toStrictEqual(undefined);
  expect(fn_echo_option_enum(wbg.Foo.A)).toStrictEqual(wbg.Foo.A);
  expect(fn_echo_option_enum(wbg.Foo.B)).toStrictEqual(wbg.Foo.B);
});
