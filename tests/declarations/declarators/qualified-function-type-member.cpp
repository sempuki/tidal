// tidal-rules: DCL-16
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
using F = int() const;
struct S { F get; int v = 2; };
int S::get() const { return v; }
F S::* pm = &S::get;
