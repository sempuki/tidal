// tidal-rules: LIST-4
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct Empty {};
struct S : Empty { int v; };
S s{1};
