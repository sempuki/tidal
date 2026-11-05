// tidal-rules: VIRT-1 VIRT-3
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-error-clang: without a ref-qualifier
// tidal-error-gcc: marked .override., but does not override
// tidal-error-edg: declared with .override. does not override
// tidal-note: Q-0394. The base function has the ref-qualifier & and the derived function has none, so the derived function does not override it, and override is not valid. Clang 22, GCC 16, and EDG reject. Clang 18 accepts and makes the derived function an override, as the gap review of the text phase 8 wrote reads the C++23 text.
struct B { virtual int f() & { return 1; } };
struct D : B { int f() override { return 2; } };
