// tidal-rules: SHF-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31 (third).
struct S { int m; };
auto f(int S::* p) { return 1 >> p; }
