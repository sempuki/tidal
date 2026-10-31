// tidal-rules: CTOR-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct M { ~M() = delete; };
struct S { M* p; M m; };
S* make() { return new S; }
