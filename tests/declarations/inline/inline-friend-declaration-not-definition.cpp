// tidal-rules: INL-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 expected reject. All compilers accept an inline friend declaration that is not a definition.
struct S { friend inline void f(); };
