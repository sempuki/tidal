// tidal-rules: UNI-7 LIST-14
// tidal-expect: reject
// tidal-error-clang: overrides prior initialization
// tidal-error-gcc: too many initializers
// tidal-error-edg: out-of-order initializers
// tidal-note: The anonymous union is one element of the list, so the designators .i and .f both initialize that one element.
struct S { int tag; union { int i; float f; }; };
S s = { .tag = 1, .i = 2, .f = 3.0f };
