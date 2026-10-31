// tidal-rules: DEFARG-6
// tidal-expect: reject
// tidal-error-clang: cannot have a default argument
// tidal-error-gcc: cannot have default arguments
// tidal-error-edg: default argument is not allowed
// tidal-note: Gap review 2026-10-30.
struct S {};
int operator+(S, S = S{});
