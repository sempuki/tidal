// tidal-rules: NEW-11
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .S\[2\].
// tidal-error-gcc: no matching function for call to .S::S\(\).
// tidal-error-edg: no default constructor exists
// tidal-note: Gap review 2026-10-31.
struct S { S(int) {} };
auto p = new S[2];
