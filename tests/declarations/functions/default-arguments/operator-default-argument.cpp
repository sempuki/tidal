// tidal-rules: DEFARG-6
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct S {};
int operator+(S, S = S{});
