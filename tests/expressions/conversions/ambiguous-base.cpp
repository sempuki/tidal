// tidal-rules: CONV-14
// tidal-expect: reject
// tidal-error-clang: ambiguous conversion from derived class
// tidal-error-gcc: is an ambiguous base of
// tidal-error-edg: base class .B. is ambiguous
// tidal-note: Gap review 2026-10-30.
struct B { int b; };
struct L : B {};
struct R : B {};
struct D : L, R {};
D d;
B* p = &d;
