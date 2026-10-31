// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
// tidal-expect-gcc: accept
// tidal-note: Q-0021. GCC ignores no_unique_address on a bit-field. Clang and EDG reject it.
struct S { [[no_unique_address]] int b : 3; };
