// tidal-rules: NEW-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { S(int) {} };
auto p = new S[2];
