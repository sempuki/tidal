// tidal-rules: LIST-8
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct E { explicit E() {} };
struct A { int v; E e; };
A a{1};
