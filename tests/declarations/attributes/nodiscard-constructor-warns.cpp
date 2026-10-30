// tidal-rules: ATTR-5
// tidal-expect: reject
// tidal-flags: -Werror
// tidal-note: Gap review 2026-10-30.
struct S { [[nodiscard]] S(int) {} };
void f() { S(1); }
