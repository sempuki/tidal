// tidal-rules: ATTR-5
// tidal-expect: reject
// tidal-error-clang: constructor declared with .nodiscard. attribute
// tidal-error-gcc: declared with attribute .nodiscard
// tidal-flags: -Werror
// tidal-note: Gap review 2026-10-30.
struct S { [[nodiscard]] S(int) {} };
void f() { S(1); }
