// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { int get(this const S& self) { return 1; } };
int (S::* p)() const = &S::get;
