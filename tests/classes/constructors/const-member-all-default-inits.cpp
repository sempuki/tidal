// tidal-rules: CTOR-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct M { int a = 1; int b = 2; };
struct S { const M m; };
S s;
