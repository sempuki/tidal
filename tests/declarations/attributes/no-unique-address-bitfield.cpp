// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-gcc: accept
// tidal-error-clang: no_unique_address. attribute only applies to non-bit-field
// tidal-error-edg: no_unique_address. does not apply to bit fields
// tidal-note: Q-0021. GCC ignores no_unique_address on a bit-field. Clang and EDG reject it.
struct S { [[no_unique_address]] int b : 3; };
